///! Scanner implementation
///!
///! Author Taketoshi Aono
///!

use regex::{Regex, Captures};
use std::result::{Result};
use std::string::String;
use parser::loader;
use parser::literal_buffer;
use parser::token::{Token, TokenKind};
use parser::sourceinfo::SourceInfo;
use parser::moduleinfo::ModuleInfo;
use std::collections::HashMap;
use std::cell::{Cell,RefCell, RefMut};


const REGEXP: &'static str = r#"(?x)
              (?P<comment>;[^\n\r]*)|
              (?P<left_paren>\()|
              (?P<right_paren>\))|
              (?P<left_brace>\{)|
              (?P<right_brace>\})|
              (?P<left_bracket>\[)|
              (?P<right_bracket>\])|
              (?P<short_lambda>\#\()|
              (?P<param_name>%(?:[1-9][0-9]*|&)?)|
              (?P<int>[0-9]+)|
              (?P<float>-?(?:[0-9]+(?:\.[0-9]+)?(?:[eE]-?[0-9]+)?|Infinity|NaN))|
              (?P<hex>0[xX][0-9a-fA-F]+)|
              (?P<bin>0[bB][10]+)|
              (?P<long>-?[0-9]+[lL]?)|
              (?P<bign>-?[0-9]+[nN])|
              (?P<charu>\\u[0-9D-Fd-f][0-9a-fA-F]{3})|
              (?P<lf>[\n\r])|
              (?P<white_spaces>[\s\t]+)|
              (?P<string>"(?:\\.|[^"])*")|
              (?P<regexp>\#"(?:\\.|[^"])*")|
              (?P<nil>nil)|
              (?P<boolean>true|false)|
              (?P<unquote_splicing>~@)|
              (?P<defef>@)|
              (?P<quote>')|
              (?P<backtick>`)|
              (?P<unquote>~)|
              (?P<tag>\^)|
              (?P<dispatch>\#[^"]+)|
              (?P<keyword>:(?:[\./]|(?:[^\^`\#'"~@:%\(\)\[\]\{\}\n\r\t\s,](?:[^\^`\#'"~@:%\(\)\[\]\{\}\n\r\t\s,]|[0-9])*)))|
              (?P<macro_keyword>::(?:[\./]|(?:[^\^`\#'"~@:%\(\)\[\]\{\}\n\r\t\s,](?:[^\^`\#'"~@:%\(\)\[\]\{\}\n\r\t\s,]|[0-9])*)))|
              (?P<symbol>(?:[\./]|(?:[^\^`\#'"~@:%\(\)\[\]\{\}\n\r\t\s,](?:[^\^`\#'"~@:%\(\)\[\]\{\}\n\r\t\s\s,]|[0-9])*)))|
              (?P<any>.)
            "#;

const LF_REGEXP: &'static str = r#"(?x)
              (?P<lf>[\n\r]) "#;


struct ParseErr {
    cause: String
}


pub struct Scanner<'a> {
    code: String,
    current_pos: Cell<i32>,
    current_line: Cell<i32>,
    index: Cell<usize>,
    len: usize,
    r: Regex,
    lf_r: Regex,
    module_info: &'a ModuleInfo,
    literal_buffer: &'a literal_buffer::LiteralBuffer<'a>
}


impl<'a> Scanner<'a> {
    pub fn new_code(module_info: &'a ModuleInfo, code: &str, lb: &'a literal_buffer::LiteralBuffer<'a>) -> Scanner<'a> {
        Scanner {
            code: code.to_string(),
            current_pos: Cell::new(1),
            current_line: Cell::new(1),
            index: Cell::new(0),
            len: code.bytes().count(),
            literal_buffer: lb,
            module_info: module_info,
            r: Regex::new(REGEXP).unwrap(),
            lf_r: Regex::new(LF_REGEXP).unwrap()
        }
    }


    pub fn new_file(module_info: &'a ModuleInfo, lb: &'a literal_buffer::LiteralBuffer<'a>) -> Scanner<'a> {
        let code = loader::load(module_info.filename());
        let len = code.bytes().count();
        Scanner {
            code: code,
            current_pos: Cell::new(1),
            current_line: Cell::new(1),
            index: Cell::new(0),
            len: len,
            literal_buffer: lb,
            module_info: module_info,
            r: Regex::new(REGEXP).unwrap(),
            lf_r: Regex::new(LF_REGEXP).unwrap()
        }
    }


    pub fn scan(&self) -> Token<'a> {
        loop {
            let cap = self.r.captures(&self.code[self.index.get()..]);
            match cap {
                Some(cap) => {
                    let matched = self.do_match(&cap);
                    let len = cap.at(0).unwrap().len();
                    self.index.set(self.index.get() + len);
                    self.current_pos.set(self.current_pos.get() + len as i32);
                    match matched {
                        Option::None => {
                            continue;
                        }
                        Option::Some(token) => {
                            return token;
                        }
                    }
                }
                None => {
                    return Token::new_eof(self.make_info());
                }
            }
        }
    }

    fn do_match(&self, cap: &Captures) -> Option<Token<'a>> {
        match cap.name("lf") {
            Some(t) => {
                self.inc_line_number();
                self.current_pos.set(1);
                return Option::None;
            }
            None => {}
        };

        match cap.name("comment") {
            Some(t) => {
                // let value_id = self.literal_buffer.get(t);
                // return Token::new_value(self.make_info(), value_id, TokenKind::Comment);
                self.count_lf(t);
                return Option::None;
            }
            None => {}
        };

        match cap.name("white_spaces") {
            Some(t) => {
                return Option::None;
            }
            None => {}
        };
        
        match cap.name("left_paren") {
            Some(t) => {
                return Option::Some(Token::new(self.make_info(), TokenKind::LeftParen));
            }
            None => {}
        };

        match cap.name("right_paren") {
            Some(t) => {
                return Option::Some(Token::new(self.make_info(), TokenKind::RightParen));
            }
            None => {}
        };

        match cap.name("left_brace") {
            Some(t) => {
                return Option::Some(Token::new(self.make_info(), TokenKind::LeftBrace));
            }
            None => {}
        };

        match cap.name("right_brace") {
            Some(t) => {
                return Option::Some(Token::new(self.make_info(), TokenKind::RightBrace));
            }
            None => {}
        };

        match cap.name("left_bracket") {
            Some(t) => {
                return Option::Some(Token::new(self.make_info(), TokenKind::LeftBracket));
            }
            None => {}
        };

        match cap.name("right_bracket") {
            Some(t) => {
                return Option::Some(Token::new(self.make_info(), TokenKind::RightBracket));
            }
            None => {}
        };

        match cap.name("int") {
            Some(t) => {
                let value_id = self.literal_buffer.get(t);
                return Option::Some(Token::new_value(self.make_info(), value_id, TokenKind::Int));
            }
            None => {}
        };

        match cap.name("float") {
            Some(t) => {
                let value_id = self.literal_buffer.get(t);
                return Option::Some(Token::new_value(self.make_info(), value_id, TokenKind::Float));
            }
            None => {}
        };

        match cap.name("hex") {
            Some(t) => {
                let value_id = self.literal_buffer.get(t);
                return Option::Some(Token::new_value(self.make_info(), value_id, TokenKind::Hex));
            }
            None => {}
        };

        match cap.name("bin") {
            Some(t) => {
                let value_id = self.literal_buffer.get(t);
                return Option::Some(Token::new_value(self.make_info(), value_id, TokenKind::Binary));
            }
            None => {}
        };

        match cap.name("long") {
            Some(t) => {
                let value_id = self.literal_buffer.get(t);
                return Option::Some(Token::new_value(self.make_info(), value_id, TokenKind::Long));
            }
            None => {}
        };

        match cap.name("bign") {
            Some(t) => {
                let value_id = self.literal_buffer.get(t);
                return Option::Some(Token::new_value(self.make_info(), value_id, TokenKind::BigNumber));
            }
            None => {}
        };

        match cap.name("charu") {
            Some(t) => {
                let value_id = self.literal_buffer.get(t);
                return Option::Some(Token::new_value(self.make_info(), value_id, TokenKind::UnicodeChar));
            }
            None => {}
        };

        match cap.name("string") {
            Some(t) => {
                let value_id = self.literal_buffer.get(t);
                self.count_lf(t);
                return Option::Some(Token::new_value(self.make_info(), value_id, TokenKind::String));
            }
            None => {}
        };

        match cap.name("regexp") {
            Some(t) => {
                let value_id = self.literal_buffer.get(t);
                return Option::Some(Token::new_value(self.make_info(), value_id, TokenKind::Regexp));
            }
            None => {}
        };
        
        match cap.name("nil") {
            Some(t) => {
                let value_id = self.literal_buffer.get(t);
                return Option::Some(Token::new_value(self.make_info(), value_id, TokenKind::Nil));
            }
            None => {}
        };

        match cap.name("boolean") {
            Some(t) => {
                let value_id = self.literal_buffer.get(t);
                return Option::Some(Token::new_value(self.make_info(), value_id, TokenKind::Boolean));
            }
            None => {}
        };

        match cap.name("unquote_splicing") {
            Some(t) => {
                return Option::Some(Token::new(self.make_info(), TokenKind::UnquoteSplicing));
            }
            None => {}
        };

        match cap.name("deref") {
            Some(t) => {
                return Option::Some(Token::new(self.make_info(), TokenKind::Deref));
            }
            None => {}
        };

        match cap.name("quote") {
            Some(t) => {
                return Option::Some(Token::new(self.make_info(), TokenKind::Quote));
            }
            None => {}
        };

        match cap.name("backtick") {
            Some(t) => {
                return Option::Some(Token::new(self.make_info(), TokenKind::Backtick));
            }
            None => {}
        };

        match cap.name("unquote") {
            Some(t) => {
                return Option::Some(Token::new(self.make_info(), TokenKind::Unquote));
            }
            None => {}
        };

        match cap.name("tag") {
            Some(t) => {
                return Option::Some(Token::new(self.make_info(), TokenKind::Tag));
            }
            None => {}
        };

        match cap.name("dispatch") {
            Some(t) => {
                let value_id = self.literal_buffer.get(t);
                return Option::Some(Token::new_value(self.make_info(), value_id, TokenKind::String));
            }
            None => {}
        };

        match cap.name("keyword") {
            Some(t) => {
                let value_id = self.literal_buffer.get(t);
                return Option::Some(Token::new_value(self.make_info(), value_id, TokenKind::Keyword));
            }
            None => {}
        };

        match cap.name("macro_keyword") {
            Some(t) => {
                let value_id = self.literal_buffer.get(t);
                return Option::Some(Token::new_value(self.make_info(), value_id, TokenKind::MacroKeyword));
            }
            None => {}
        };

        match cap.name("short_lambda") {
            Some(t) => {
                return Option::Some(Token::new(self.make_info(), TokenKind::ShortLambdaBegin));
            },
            None => {}
        }

        match cap.name("symbol") {
            Some(t) => {
                let value_id = self.literal_buffer.get(t);
                return Option::Some(Token::new_value(self.make_info(), value_id, TokenKind::Symbol));
            }
            None => {}
        };

        match cap.name("param_name") {
            Some(t) => {
                let value_id = self.literal_buffer.get(t);
                return Option::Some(Token::new_value(self.make_info(), value_id, TokenKind::ParamName));
            }
            None => {}
        };
        
        return Option::Some(Token::new_invalid(self.make_info()));
    }
    
    
    fn inc_line_number(&self) {
        self.current_line.set(self.current_line.get() + 1);
    }
    

    fn get_line_number(&self) -> i32 {
        self.current_line.get()
    }


    fn make_info(&self) -> SourceInfo<'a> {
        SourceInfo::new(self.current_pos.get(), self.get_line_number(), self.module_info)
    }
    

    fn count_lf(&self, s: &str) {
        for c in self.lf_r.captures_iter(s) {
            match c.name("lf") {
                Some(t) => self.inc_line_number(),
                None => break
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_scan_1() {
//         use super::*;
//         let scanner = Scanner::new(r#"
//           (defn test []
//             (+ 1 1))
//         "#);
//         scanner.scan(|token: Token| {
//             println!("{}", token);
//         });
//     }
// }
