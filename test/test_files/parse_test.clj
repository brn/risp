;;;; Browser test runner
;;; Author Taketoshi Aono
;;;
;;; Seleniumを実行するrunnerを定義する。
;;;

(ns cats-client-tester.runner
  (:require [clojure.tools.logging :as log]
            [clojure.core.async :as async]
            [clojure.core.cache :as cache]
            [clojure.string :as str]
            [clj-webdriver.core :as webdriver-core]
            [cats-client-tester.browser :as br]
            [cats-client-tester.driver-util :as driver-util]
            [cats-client-tester.logger :as logger]
            [cats-client-tester.url-selector :as url-selector]
            [cats-client-tester.worker :as w]))


;; webdriverとロガーを作成する。
(defn- create-driver [browser] {:driver (driver-util/init-driver browser)
                                :logger (logger/init-logger browser)})


(defn- init-cache
  "キャッシュにドライバを設定する。
   evict?  - boolean
   c       - clojure.core.cache.Cache
   browser - cats-client-tester.browser.Browser"
  ([evict? c browser ctn-id]
   (let [browser-type (keyword (str (br/nameof browser) "-" ctn-id))
         new-cache (if evict? (cache/evict c browser-type) c)]
     (if-not (cache/has? new-cache browser-type)
       (cache/miss new-cache browser-type (create-driver browser))
       c))))


(defn- driver-from-cache
  "キャッシュからwebdriverを取り出す
   c       - clojure.core.cache.Cache
   key     - keyword
   return  - clj-webdriver.driver.Driver"
  [c key] (:driver (cache/lookup c key)))


(defn- quit
  "webdriverを終了する。
   c       - clojure.core.cache.Cache
   browser - cats-client-tester.browser.Browser"
  [c ctn-id browser] (webdriver-core/quit
                      (driver-from-cache c (keyword (str (br/nameof browser) "-" ctn-id)))))


(defn- start-test
  "ブラウザによるテストを開始する。
   url     - string
   browser - Browser
   logger  - ch.qos.logback.classic Logger"
  ([url c browser ctn-id]
   (let [new-cache (init-cache false c browser ctn-id)
         key (keyword (str (br/nameof browser) "-" ctn-id))
         {driver :driver browser-logger :logger} (cache/lookup new-cache key)]
     (try
       (driver-util/check-whiteout driver url browser-logger)
       (driver-util/save-screen-shot driver browser url ctn-id)
       (driver-util/log-errors driver browser url browser-logger)
       new-cache
       (catch Exception e
         (quit new-cache ctn-id browser)
         (init-cache true new-cache browser ctn-id))))))


(defn- parse-line
  "URLリストのファイルの一行をパースする。
   line   - str
   return - {:ctn-id str :url str}"
  [line]
  (let [splited (str/split line #"\s+")]
    {:ctn-id (first splited)
     :url    (first (rest splited))}))


(defn- file->assoc-vec
  "ファイルを連想配列のvecに変換する。
   以下のような構造になる。
   [[:mst_container_id [url ...]] ...]
   lines - [str...]"
  ([lines]
   (into [] (reduce
             (fn [ret line]
               (let [{ctn-id :ctn-id url :url} (parse-line line)
                     ctn-id-key (keyword ctn-id)
                     val (ctn-id-key ret)]
                 (if (seq val)
                   (let [n (into val [url])]
                     (assoc ret ctn-id-key n))
                   (assoc ret ctn-id-key [url]))))
             {} lines))))


(defn- run-each-browser
  "各ブラウザを並列に実行する。
   browsers - [cats-client-tester.browser.Browser...]
   c        - clojure.core.cache.Cache
   id       - int
   ctn-id   - int
   url-list - [str ...]"
  ([browsers c id [ctn-id url-list]]
   (doall
    (pmap
     (fn [browser]
       (loop [urls (rest url-list)
              url  (first url-list)
              dc   c]
         (let [new-cache (start-test url dc browser ctn-id)]
           (if (seq urls)
             (recur (rest urls) (first urls) new-cache)
             (quit new-cache ctn-id browser))))) browsers))))


(defn run
  "DBか、指定されたファイルからurlのリストをパースして取り出し、
   テストを実行する
   各テストはサイトID毎に、スレッドで実行される。
   browsers - cats-client-tester.browser.Browser
   filename(opt) - str"
  ([browsers]
   (let [mst-container-ids (url-selector/select-mst-container-ids)
         c (cache/fifo-cache-factory {})
         worker (w/init-worker #(run-each-browser browsers c %1 %2))]
     (doseq [mst-container-id mst-container-ids]
       (let [url-list (url-selector/select-url-list (:mst_container_id mst-container-id))]
         (when (seq url-list)
           (w/submit worker [mst-container-id (map #(:url %) url-list)]))))
     (w/shutdown worker)))
  ([browsers filename]
   (with-open [r (clojure.java.io/reader filename)]
     (let [c (cache/fifo-cache-factory {})
           assoc-v (file->assoc-vec (line-seq r))
           worker (w/init-worker #(run-each-browser browsers c %1 %2))]
       (doseq [item assoc-v]
         (w/submit worker item))
       (w/shutdown worker)))))

