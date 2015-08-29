///! Source code loader
///! Author Taketoshi Aono
///!

use std;
use std::io::Read;
use std::result::Result;

pub fn load(src: &str) -> String {
    let mut mf = std::fs::File::open(src);
    match mf {
        Result::Ok(mut f) => {
            let mut s = String::new();
            f.read_to_string(&mut s);
            s
        },
        Result::Err(err) => panic!(err)
    }
}
