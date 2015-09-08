//! Token structure definition
//! author Taketoshi Aono
//!

use std;
use std::collections::HashMap;
use parser::sourceinfo::SourceInfo;

/// Token kind definitions.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    Root,
    Invalid,
    Ignore,
    Symbol,
    Lambda,
    ShortLambdaBegin,
    Let,
    If,
    Def,
    DefMacro,
    Keyword,
    MacroKeyword,
    Float,
    Int,
    Long,
    Hex,
    Binary,
    BigNumber,
    UnicodeChar,
    String,
    Regexp,
    Nil,
    Boolean,
    ParamName,
    LeftParen,
    Comment,
    UnquoteSplicing,
    Deref,
    Ref,
    Backtick,
    Unquote,
    Tag,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Quote,
    QuoteRm,
    KQuote,
    Colon,
    Eof
}


const LEFT_PAREN: char    = '(';
const RIGHT_PAREN: char   = ')';
const LEFT_BRACKET: char  = '[';
const RIGHT_BRACKET: char = ']';
const LEFT_BRACE: char    = '{';
const RIGHT_BRACE: char   = '}';
const QUOTE: char         = '\'';
const COLON: char         = ':';


trait Oneof<T> {
    fn oneof(&self, values: &[T]) -> bool;
}


#[derive(Copy, Clone)]
pub struct Token<'a> {
    info: SourceInfo<'a>,
    value: i64,
    kind: TokenKind
}


impl<'a> std::fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Token({:?},{:?},{:?})", self.pos(), self.line(), self.kind)
    }
}


impl TokenKind {
    /// Create TokenKind from char.
    /// # Example
    ///
    /// ```
    /// use risp::parser::token::TokenKind;
    /// let left_paren = TokenKind::from_char('(');
    /// let right_paren = TokenKind::from_char(')');
    /// let left_brace = TokenKind::from_char('{');
    /// let right_brace = TokenKind::from_char('}');
    /// let left_bracket = TokenKind::from_char('[');
    /// let right_bracket = TokenKind::from_char(']');
    /// let colon = TokenKind::from_char(':');
    /// let quote = TokenKind::from_char('\'');
    /// let inval = TokenKind::from_char('$');
    /// 
    /// assert_eq!(left_paren, TokenKind::LeftParen);
    /// assert_eq!(right_paren, TokenKind::RightParen);
    /// assert_eq!(left_brace, TokenKind::LeftBrace);
    /// assert_eq!(right_brace, TokenKind::RightBrace);
    /// assert_eq!(left_bracket, TokenKind::LeftBracket);
    /// assert_eq!(right_bracket, TokenKind::RightBracket);
    /// assert_eq!(colon, TokenKind::Colon);
    /// assert_eq!(quote, TokenKind::Quote);
    /// assert_eq!(inval, TokenKind::Invalid);
    /// ```
    pub fn from_char(c: char) -> TokenKind {
        match c {
            LEFT_PAREN    => TokenKind::LeftParen,
            RIGHT_PAREN   => TokenKind::RightParen,
            LEFT_BRACKET  => TokenKind::LeftBracket,
            RIGHT_BRACKET => TokenKind::RightBracket,
            LEFT_BRACE    => TokenKind::LeftBrace,
            RIGHT_BRACE   => TokenKind::RightBrace,
            QUOTE         => TokenKind::Quote,
            COLON         => TokenKind::Colon,
            _             => TokenKind::Invalid
        }
    }
}


impl Oneof<TokenKind> for TokenKind {
    /// Check whether one of kinds are matched with token instance.
    fn oneof(&self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if *kind == *self {
                return true;
            }
        }
        false
    }
}


/// Token implementations
impl<'a> Token<'a> {
    /// Initialize Token.
    /// # Example
    ///
    /// ```
    /// use risp::parser::token::{Token, TokenKind};
    /// use risp::parser::sourceinfo::SourceInfo;
    /// let token = Token::new(SourceInfo::new(1, 1), TokenKind::LeftParen);
    /// assert_eq!(token.col_num(), 1);
    /// assert_eq!(token.line_num(), 1);
    /// assert_eq!(token.kind(), TokenKind::LeftParen);
    /// assert_eq!(token.is(TokenKind::LeftParen), true);
    /// ```
    pub fn new(info: SourceInfo<'a>, kind: TokenKind) -> Token<'a> {
        Token {
            info  : info,
            value : -1,
            kind  : kind
        }
    }

    /// Initialize Token from char.
    /// # Example
    ///
    /// ```
    /// use risp::parser::token::{Token, TokenKind};
    /// use risp::parser::sourceinfo::SourceInfo;
    /// let token = Token::new_from_char(SourceInfo::new(1, 1), '(');
    /// assert_eq!(token.col(), 1);
    /// assert_eq!(token.line(), 1);
    /// assert_eq!(token.kind(), TokenKind::LeftParen);
    /// assert_eq!(token.is(TokenKind::LeftParen), true);
    /// ```
    pub fn new_from_char(info: SourceInfo<'a>, token: char) -> Token<'a> {
        Token {
            info  : info,
            value : -1,
            kind  : TokenKind::from_char(token)
        }
    }
    
    pub fn new_value(info: SourceInfo<'a>, value: i64, kind: TokenKind) -> Token<'a> {
        Token {
            info  : info,
            value : value,
            kind  : kind
        }
    }

    pub fn new_eof(info: SourceInfo<'a>) -> Token<'a> {
        Token {
            info: info,
            value: -1,
            kind: TokenKind::Eof
        }
    }

    pub fn new_invalid(info: SourceInfo<'a>) -> Token<'a> {
        Token {
            info  : info,
            value : -1,
            kind  : TokenKind::Invalid
        }
    }
    
    pub fn pos(&self) -> i32 {
        self.info.pos()
    }
    
    pub fn line(&self) -> i32 {
        self.info.line()
    }

    pub fn info(&self) -> SourceInfo {
        self.info
    }
    
    pub fn kind(self) -> TokenKind {
        self.kind
    }

    pub fn value(&self) -> i64 {
        self.value
    }

    pub fn is(&self, kind: TokenKind) -> bool {
        self.kind == kind
    }
}


impl<'a> Oneof<TokenKind> for Token<'a> {
    fn oneof(&self, kinds: &[TokenKind]) -> bool {
        self.kind.oneof(kinds)
    }
}


// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_token_kind_one_of() {
//         use super::*;
//         use super::Oneof;
//         let matched = TokenKind::LeftParen;
//         let unmatched = TokenKind::Quote;
//         assert_eq!(matched.oneof(&[TokenKind::LeftBracket, TokenKind::LeftBrace, TokenKind::LeftParen]), true);
//         assert_eq!(unmatched.oneof(&[TokenKind::LeftBracket, TokenKind::LeftBrace, TokenKind::LeftParen]), false);
//     }

//     #[test]
//     fn test_token_one_of() {
//         use super::*;
//         use super::Oneof;
//         let matched = Token::new(1, 1, TokenKind::LeftParen);
//         let unmatched = Token::new(1, 1, TokenKind::Quote);
//         assert_eq!(matched.oneof(&[TokenKind::LeftBracket, TokenKind::LeftBrace, TokenKind::LeftParen]), true);
//         assert_eq!(unmatched.oneof(&[TokenKind::LeftBracket, TokenKind::LeftBrace, TokenKind::LeftParen]), false);
//     }
// }
