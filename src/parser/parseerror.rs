///! Parsing Error definition.
///!
///! Author Taketoshi Aono

use parser::token::Token;
use std::fmt::{Display, Result, Formatter};

pub struct ParseError<'a> {
    message: String,
    token: Token<'a>
}

impl<'a> Display for ParseError<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}:{}", self.message, self.token.info())
    }
}

impl<'a> ParseError<'a> {
    pub fn new(message: &str, token: Token<'a>) -> ParseError<'a> {
        ParseError {
            message: message.to_string(),
            token: token
        }
    }
}
