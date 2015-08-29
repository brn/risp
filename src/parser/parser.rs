///! Parser implementation
///! Author Taketoshi Aono
///!

use std::result::{Result};

use parser::token::{Token, TokenKind};
use parser::moduleinfo::ModuleInfo;
use parser::scanner::Scanner;
use parser::parseerror::ParseError;
use parser::sourceinfo::SourceInfo;
use parser::literal_buffer::LiteralBuffer;
use internal::ast::{Ast, FormKind};
use internal::heap::zone::{ZoneAllocator};
use std::rc::Rc;

macro_rules! check_token {
    ($ast:ident, $token:expr, $rest:block) => {
        match $token.kind() {
            TokenKind::Eof => {
                return Result::Ok($ast);
            },
            TokenKind::Invalid => {
                return Result::Err(ParseError::new("Invlaid token.", $token));
            }
            _ => $rest
        }
    }
}


type Node<'a> = Ast<'a>;
pub type ParseResult<'a> = Result<&'a Node<'a>, ParseError<'a>>;


pub struct Parser<'a> {
    module_info: &'a ModuleInfo,
    scanner: Scanner<'a>,
    zone_allocator: &'a ZoneAllocator
}

impl<'a> Parser<'a> {
    pub fn new_from_file(module_info: &'a ModuleInfo, literal_buffer: &'a LiteralBuffer<'a>, zone_allocator: &'a ZoneAllocator) -> Parser<'a> {
        Parser {
            module_info: module_info,
            scanner: Scanner::new_file(module_info, literal_buffer),
            zone_allocator: zone_allocator
        }
    }


    pub fn new_from_code(module_info: &'a ModuleInfo, code: &str, literal_buffer: &'a LiteralBuffer<'a>, zone_allocator: &'a ZoneAllocator) -> Parser<'a> {
        Parser {
            module_info: module_info,
            scanner: Scanner::new_code(module_info, code, literal_buffer),
            zone_allocator: zone_allocator
        }
    }


    pub fn parse(&self) -> ParseResult<'a> {
        let token = Token::new(SourceInfo::new(1, 1, self.module_info), TokenKind::Root);
        let module = Ast::new_module(self.zone_allocator, self.module_info);
        loop {
            let token = self.scanner.scan();
            check_token!(module, token, {
                match token.kind() {
                    TokenKind::LeftParen => {
                        match self.parse_form() {
                            Ok(ast) => ast_add_child!(module, ast),
                            Err(e) => {return Result::Err(e);}
                        }
                    }
                    _ => {
                        return Result::Err(ParseError::new("Invlaid token.", token));
                    }
                }
            })
        }
    }


    fn parse_form(&self) -> ParseResult<'a> {
        let mut next = self.scanner.scan();
        let form = Ast::new_form(self.zone_allocator, FormKind::List);
        loop {
            check_token!(form, next, {
                match next.kind() {
                    TokenKind::RightParen => {
                        return Result::Ok(form);
                    }
                    _ => {
                        match self.do_parse_form(form, next) {
                            Ok(ast) => {},
                            Err(e) => {return Result::Err(e);}
                        }
                    }
                }
            });
            next = self.scanner.scan();
        }
    }


    fn do_parse_form(&self, form: &'a Ast<'a>, token: Token<'a>) -> ParseResult<'a>{
        let next = token;
        check_token!(form, next, {
            match next.kind() {
                TokenKind::LeftParen => {
                    match self.parse_form() {
                        Ok(ast) => ast_add_child!(form, ast),
                        Err(e) => {return Result::Err(e);}
                    }
                }
                TokenKind::ShortLambdaBegin => {
                    match self.parse_short_lambda() {
                        Ok(ast) => ast_add_child!(form, ast),
                        Err(e) => {return Result::Err(e);}
                    }
                }
                TokenKind::LeftBracket => {
                    match self.parse_vector(next) {
                        Ok(ast) => ast_add_child!(form, ast),
                        Err(e) => {return Result::Err(e);}
                    }
                }
                TokenKind::LeftBrace => {
                    match self.parse_map(next) {
                        Ok(ast) => ast_add_child!(form, ast),
                        Err(e) => {return Result::Err(e);}
                    }
                }
                _ => {
                    match self.parse_literal(next) {
                        Ok(ast) => ast_add_child!(form, ast),
                        Err(e) => {return Result::Err(e);}
                    }
                }
            }
        });
        Result::Ok(form)
    }


    fn parse_vector(&self, token: Token<'a>) -> ParseResult<'a> {
        let vector = Ast::new_form(self.zone_allocator, FormKind::Vector);
        loop {
            let next = self.scanner.scan();
            check_token!(vector, next, {
                match next.kind() {
                    TokenKind::RightBracket => {
                        return Result::Ok(vector);
                    }
                    _ => {
                        match self.do_parse_form(vector, next) {
                            Ok(ast) => {},
                            Err(e) => {return Result::Err(e);}
                        }
                    }
                }
            })
        }
    }


    fn parse_map(&self, token: Token<'a>) -> ParseResult<'a> {
        let map = Ast::new_form(self.zone_allocator, FormKind::Map);
        loop {
            let next = self.scanner.scan();
            check_token!(map, next, {
                match next.kind() {
                    TokenKind::RightBrace => {
                        return Result::Ok(map);
                    }
                    _ => {
                        match self.do_parse_form(map, next) {
                            Ok(ast) => {},
                            Err(e) => {return Result::Err(e);}
                        }
                    }
                }
            })
        }
    }


    fn parse_short_lambda(&self) -> ParseResult<'a> {
        let lambda = Ast::new_form(self.zone_allocator, FormKind::Lambda);
        loop {
            let next = self.scanner.scan();
            check_token!(lambda, next, {
                match next.kind() {
                    TokenKind::ParamName => {
                        ast_add_child!(lambda, Ast::new_literal(self.zone_allocator, next))
                    },
                    TokenKind::RightParen => {
                        return Result::Ok(lambda);
                    }
                    _ => {
                        match self.do_parse_form(lambda, next) {
                            Ok(ast) => {},
                            Err(e) => {return Result::Err(e);}
                        }
                    }
                }
            })
        }
    }


    fn parse_literal(&self, token: Token<'a>) -> ParseResult<'a> {
        Ok(Ast::new_literal(self.zone_allocator, token))
    }
}
