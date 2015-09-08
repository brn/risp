///! Parser implementation
///! Author Taketoshi Aono
///!

use std::cell::{Cell};
use std::result::{Result};

use parser::token::{Token, TokenKind};
use parser::moduleinfo::ModuleInfo;
use parser::scanner::Scanner;
use parser::parseerror::ParseError;
use parser::sourceinfo::SourceInfo;
use parser::literal_buffer::LiteralBuffer;
use internal::ast::*;
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


macro_rules! get_token_value {
    ($s:expr, $token:expr) => {$s.literal_buffer.find($token.value())}
}


type Node<'a> = Ast<'a>;
pub type ParseResult<'a> = Result<&'a Node<'a>, ParseError<'a>>;


pub struct ScopeHandler<'a> {
    current_scope: Cell<&'a Scope<'a>>,
    zone_allocator: &'a ZoneAllocator,
    id: Cell<i32>,
}


impl<'a> ScopeHandler<'a> {
    pub fn new(za: &'a ZoneAllocator) -> ScopeHandler<'a> {
        let id = 0;
        ScopeHandler {
            current_scope: Cell::new(Scope::new(za, id)),
            zone_allocator: za,
            id: Cell::new(id)
        }
    }


    pub fn enter<T, U>(&self, mut f: T) -> U
        where T: FnMut(&'a Scope<'a>) -> U {
            self.id.set(self.id.get() + 1);
            let scope = Scope::new(self.zone_allocator, self.id.get());
            scope.set_parent(self.current_scope.get());
            self.current_scope.set(scope);
            let ret = f(scope);
            self.current_scope.set(scope.parent().unwrap());
            ret
        }
    
    
    pub fn scope(&self) -> &'a Scope<'a> {
        self.current_scope.get()
    }
}


pub struct Parser<'a> {
    module_info: &'a ModuleInfo,
    literal_buffer: &'a LiteralBuffer<'a>,
    scanner: Scanner<'a>,
    scope_handler: ScopeHandler<'a>,
    zone_allocator: &'a ZoneAllocator
}


impl<'a> Parser<'a> {
    pub fn new_from_file(module_info: &'a ModuleInfo, literal_buffer: &'a LiteralBuffer<'a>, zone_allocator: &'a ZoneAllocator) -> Parser<'a> {
        Parser {
            module_info: module_info,
            scanner: Scanner::new_file(module_info, literal_buffer),
            literal_buffer: literal_buffer,
            scope_handler: ScopeHandler::new(zone_allocator),
            zone_allocator: zone_allocator
        }
    }


    pub fn new_from_code(module_info: &'a ModuleInfo, code: &str, literal_buffer: &'a LiteralBuffer<'a>, zone_allocator: &'a ZoneAllocator) -> Parser<'a> {
        Parser {
            module_info: module_info,
            scanner: Scanner::new_code(module_info, code, literal_buffer),
            literal_buffer: literal_buffer,
            scope_handler: ScopeHandler::new(zone_allocator),
            zone_allocator: zone_allocator
        }
    }


    pub fn parse(&self) -> ParseResult<'a> {
        let token = Token::new(SourceInfo::new(1, 1, self.module_info), TokenKind::Root);
        let module = Ast::new_module(self.zone_allocator, self.module_info, self.scope_handler.scope());
        loop {
            let token = self.scanner.scan();
            check_token!(module, token, {
                match token.kind() {
                    TokenKind::LeftParen => {
                        match self.parse_form(token) {
                            Ok(ast) => module.add_child(ast),
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
    

    fn parse_form(&self, token: Token<'a>) -> ParseResult<'a> {
        let mut next = self.scanner.scan();
        let form;

        match next.kind() {
            TokenKind::Eof => {
                return Err(ParseError::new("Unexpected end of input", next));
            }
            TokenKind::Invalid => {
                return Err(ParseError::new("Invalid token.", next));
            }
            TokenKind::Def => {
                return self.parse_def(token);
            },
            TokenKind::DefMacro => {
                return self.parse_defmacro();
            }
            TokenKind::Quote => {
                return self.parse_quote(Ast::new_quote(self.zone_allocator, token));
            }
            TokenKind::Let => {
                return self.parse_let(token);
            }
            TokenKind::Lambda => {
                return self.parse_lambda(token);
            },
            TokenKind::If => {
                return self.parse_if(Ast::new_if(self.zone_allocator, token));
            }
            TokenKind::RightParen => {
                return Result::Ok(Ast::new_list(self.zone_allocator, token));
            }
            _ => {
                form = Ast::new_list(self.zone_allocator, token);
                match self.do_parse_form(form, next, |ast: &'a Ast<'a>| form.add_child(ast)) {
                    Ok(ast) => {},
                    Err(e) => {return Result::Err(e);}
                }
            }
        }
        next = self.scanner.scan();

        loop {
            check_token!(form, next, {
                match next.kind() {
                    TokenKind::RightParen => {
                        return Result::Ok(form);
                    }
                    _ => {
                        match self.do_parse_form(form, next, |ast: &'a Ast<'a>| form.add_child(ast)) {
                            Ok(ast) => {},
                            Err(e) => {return Result::Err(e);}
                        }
                    }
                }
            });
            next = self.scanner.scan();
        }
    }


    fn parse_let(&self, token: Token<'a>) -> ParseResult<'a> {
        return self.scope_handler.enter(|scope: &'a Scope<'a>| -> ParseResult<'a> {
            let let_form = Ast::new_let(self.zone_allocator, token, scope);
            let mut next_token = self.scanner.scan();
            let mut binding: &'a Ast<'a>;
            
            if !next_token.is(TokenKind::LeftBracket) {
                return Err(ParseError::new("let expect vector binding form.", next_token));
            }

            next_token = self.scanner.scan();

            loop {
                match self.parse_literal(next_token) {
                    Ok(ast) => {
                        match ast {
                            &Ast::Symbol(ref s) => {
                                match ast.symbol_mode() {
                                    SymbolMode::Unresolved => {
                                        ast.set_symbol_mode(SymbolMode::Var(SymbolDepth::Origin));
                                    }
                                    _ => {}
                                }
                                binding = ast;
                                scope.intern(binding);
                            }
                            _ => {return Err(ParseError::new("Invalid binding form", next_token));}
                        }
                    }
                    Err(e) => {return Err(e);}
                }

                next_token = self.scanner.scan();
                match self.do_parse_form(let_form, next_token, |ast: &'a Ast<'a>| {
                    binding.bind_to_symbol(ast);
                    let_form.add_let_binding((binding, ast));
                }) {
                    Err(e) => {return Err(ParseError::new("let form expected symbol-value pair.", next_token));}
                    _ => {}
                }

                next_token = self.scanner.scan();
                if next_token.is(TokenKind::RightBracket) {
                    break;
                }
            }

            next_token = self.scanner.scan();
            loop {
                match self.do_parse_form(let_form, next_token, |ast: &'a Ast<'a>| let_form.add_let_body(ast)) {
                    Err(e) => {return Err(e);},
                    _ => {}
                }

                next_token = self.scanner.scan();
                if next_token.is(TokenKind::RightParen) {
                    return Ok(let_form);
                }
            }
        });
    }


    fn parse_def(&self, token: Token<'a>) -> ParseResult<'a> {
        let def_ast = Ast::new_def(self.zone_allocator, token);        
        let mut next_token = self.scanner.scan();
        let binding: &'a Ast<'a>;
        
        match self.parse_literal(next_token) {
            Ok(ast) => {
                match ast {
                    &Ast::Symbol(ref s) => {
                        ast.set_symbol_mode(SymbolMode::Var(SymbolDepth::Origin));
                        self.intern_scope(ast);
                        binding = ast;
                    }
                    _ => {return Err(ParseError::new("The first argument of def must be a symbol.", next_token));}
                }
                def_ast.set_def_name(ast);
            },
            Err(e) => {return Err(e);}
        }

        next_token = self.scanner.scan();
        match self.do_parse_form(def_ast, next_token, |ast: &'a Ast<'a>| {
            def_ast.set_def_expr(ast);
            binding.bind_to_symbol(ast);
        }) {
            Err(e) => {return Err(e);}
            _ => {}
        }

        next_token = self.scanner.scan();
        if next_token.kind() == TokenKind::RightParen {
            return Ok(def_ast);
        }

        return Err(ParseError::new("Def accept 2 argument, ')' expected.", next_token));
    }
    

    fn parse_lambda(&self, token: Token<'a>) -> ParseResult<'a> {
        return self.scope_handler.enter(|scope: &'a Scope<'a>| -> ParseResult<'a> {
            let lambda = Ast::new_lambda(self.zone_allocator, token, scope);
            
            let mut token = self.scanner.scan();
            if token.kind() != TokenKind::LeftBracket {
                return Err(ParseError::new("Lambda expected parameter defintion as a vector.", token));
            }

            let mut index = 0;
            loop {
                token = self.scanner.scan();
                if token.kind() == TokenKind::RightBracket {
                    break;
                }
                match self.do_parse_form(lambda, token, |ast: &'a Ast<'a>| {
                    match ast {
                        &Ast::Symbol(ref s) => {
                            self.intern_scope(ast);
                            ast.set_symbol_mode(SymbolMode::Parameter{index: index, depth: SymbolDepth::Origin});
                        }
                        _ => {}
                    }
                    lambda.add_lambda_arg(ast)
                }) {
                    Err(e) => {return Err(e);},
                    _ => {}
                }
                index += 1;
            }

            token = self.scanner.scan();

            loop {
                match self.do_parse_form(lambda, token, |ast: &'a Ast<'a>| lambda.add_lambda_body(ast)) {
                    Err(e) => {return Err(e);}
                    _ => {}
                }

                token = self.scanner.scan();
                if token.kind() == TokenKind::RightParen {
                    return Ok(lambda);
                }
            }
        });
    }


    fn parse_defmacro(&self) -> ParseResult<'a> {
        return self.scope_handler.enter(|scope: &'a Scope<'a>| -> ParseResult<'a> {
            let mut token = self.scanner.scan();

            if token.kind() != TokenKind::Symbol {
                return Err(ParseError::new("defmacro name expected symbol.", token));
            }

            let defmacro;
            match self.parse_literal(token) {
                Ok(ast) => {
                    match ast {
                        &Ast::Symbol(ref s) => {
                            ast.set_symbol_mode(SymbolMode::Var(SymbolDepth::Origin));
                        }
                        _ => {return Err(ParseError::new("The first argument of defmacro must be a symbol.", token));}
                    }
                    defmacro = Ast::new_defmacro(self.zone_allocator, token, ast, scope);
                    ast.bind_to_symbol(defmacro);
                },
                Err(e) => {return Err(e);}
            }
            
            
            let mut token = self.scanner.scan();
            if token.kind() != TokenKind::LeftBracket {
                return Err(ParseError::new("defmacro expected parameter defintion as a vector.", token));
            }

            let mut index = 0;
            loop {
                token = self.scanner.scan();
                if token.kind() == TokenKind::RightBracket {
                    break;
                }
                match self.do_parse_form(defmacro, token, |ast: &'a Ast<'a>| {
                    match ast {
                        &Ast::Symbol(ref s) => {
                            self.intern_scope(ast);
                            ast.set_symbol_mode(SymbolMode::Parameter{index: index, depth: SymbolDepth::Origin});
                        }
                        _ => {}
                    }
                    defmacro.add_macro_arg(ast)
                }) {
                    Err(e) => {return Err(e);},
                    _ => {}
                }
                index += 1;
            }


            token = self.scanner.scan();
            match self.do_parse_form(defmacro, token, |ast: &'a Ast<'a>| defmacro.add_macro_body(ast)) {
                Err(e) => {return Err(e);}
                _ => {}
            }

            token = self.scanner.scan();
            if token.kind() == TokenKind::RightParen {
                return Ok(defmacro);
            }

            Err(ParseError::new("defmacro close paren [)] expected.", token))
        });
    }


    fn parse_if(&self, if_form: &'a Ast<'a>) -> ParseResult<'a> {
        let mut token = self.scanner.scan();
        match self.do_parse_form(if_form, token, |ast: &'a Ast<'a>| if_form.set_cond(ast)) {
            Err(e) => {return Err(e);}
            _ => {}
        }

        token = self.scanner.scan();
        match self.do_parse_form(if_form, token, |ast: &'a Ast<'a>| if_form.set_then_body(ast)) {
            Err(e) => {return Err(e);}
            _ => {}
        }

        token = self.scanner.scan();
        if token.kind() != TokenKind::RightParen {
            match self.do_parse_form(if_form, token, |ast: &'a Ast<'a>| if_form.set_else_body(ast)) {
                Err(e) => {return Err(e);},
                _ => {}
            }
        }

        token = self.scanner.scan();

        if token.kind() == TokenKind::RightParen {
            return Ok(if_form);
        }

        Err(ParseError::new("')' expected.", token))
    }


    fn parse_quote(&self, quote: &'a Ast<'a>) -> ParseResult<'a> {
        let mut token = self.scanner.scan();
        match self.do_parse_form(quote, token, |ast: &'a Ast<'a>| quote.set_quote_expr(ast)) {
            Err(e) => {return Err(e)},
            _ => {}
        }

        token = self.scanner.scan();

        if token.kind() == TokenKind::RightParen {
            return Ok(quote);
        }

        Err(ParseError::new("quote expected only one argument.", token))
    }


    fn do_parse_form<T>(&self, form: &'a Ast<'a>, token: Token<'a>, mut add: T) -> ParseResult<'a> where
        T: FnMut(&'a Ast<'a>) {
        let next = token;
        check_token!(form, next, {
            match next.kind() {
                TokenKind::LeftParen => {
                    match self.parse_form(next) {
                        Ok(ast) => add(ast),
                        Err(e) => {return Result::Err(e);}
                    }
                }
                TokenKind::ShortLambdaBegin => {
                    match self.parse_short_lambda(next) {
                        Ok(ast) => add(ast),
                        Err(e) => {return Result::Err(e);}
                    }
                }
                TokenKind::LeftBracket => {
                    match self.parse_vector(next) {
                        Ok(ast) => add(ast),
                        Err(e) => {return Result::Err(e);}
                    }
                }
                TokenKind::LeftBrace => {
                    match self.parse_map(next) {
                        Ok(ast) => add(ast),
                        Err(e) => {return Result::Err(e);}
                    }
                }
                TokenKind::Tag => {
                    match self.parse_tag(next) {
                        Ok(ast) => add(ast),
                        Err(e) => {return Result::Err(e);}
                    }
                }
                _ => {
                    match self.parse_literal(next) {
                        Ok(ast) => add(ast),
                        Err(e) => {return Result::Err(e);}
                    }
                }
            }
        });
        Result::Ok(form)
    }


    fn parse_vector(&self, token: Token<'a>) -> ParseResult<'a> {
        let vector = Ast::new_vector(self.zone_allocator, token);
        loop {
            let next = self.scanner.scan();
            check_token!(vector, next, {
                match next.kind() {
                    TokenKind::RightBracket => {
                        return Result::Ok(vector);
                    }
                    _ => {
                        match self.do_parse_form(vector, next, |ast: &'a Ast<'a>| vector.add_child(ast)) {
                            Ok(ast) => {},
                            Err(e) => {return Result::Err(e);}
                        }
                    }
                }
            })
        }
    }


    fn parse_map(&self, token: Token<'a>) -> ParseResult<'a> {
        let map = Ast::new_map(self.zone_allocator, token);
        loop {
            let next = self.scanner.scan();
            check_token!(map, next, {
                match next.kind() {
                    TokenKind::RightBrace => {
                        match map.children() {
                            Some(children) => {
                                if children.len() % 2 != 0 {
                                    match map.token() {
                                        Some(token) => {
                                            return Result::Err(ParseError::new("map expected key-value pair.", token));
                                        }
                                        _ => {}
                                    }
                                }
                            },
                            _ => {}
                        }
                        return Result::Ok(map);
                    }
                    _ => {
                        match self.do_parse_form(map, next, |ast: &'a Ast<'a>| map.add_child(ast)) {
                            Ok(ast) => {},
                            Err(e) => {return Result::Err(e);}
                        }
                    }
                }
            })
        }
    }


    fn parse_tag(&self, token: Token<'a>) -> ParseResult<'a> {
        let form = Ast::new_tag(self.zone_allocator, token);
        let mut next = self.scanner.scan();

        match self.do_parse_form(form, next, |ast: &'a Ast<'a>| form.add_child(ast)) {
            Ok(ast) => {},
            Err(e) => {return Result::Err(e)}
        }
        Result::Ok(form)
    }
    

    fn parse_short_lambda(&self, token: Token<'a>) -> ParseResult<'a> {
        let lambda = Ast::new_lambda_sugar(self.zone_allocator, token);
        loop {
            let next = self.scanner.scan();
            check_token!(lambda, next, {
                match next.kind() {
                    TokenKind::ParamName => {
                        let s = get_token_value!(self, next);
                        match s.parse::<i32>() {
                            Ok(i) => {
                                lambda.add_child(Ast::new_lambda_param(self.zone_allocator, next, i));
                            },
                            Err(_) => {
                                return Result::Err(ParseError::new("Invalid Lambda parameter.", next));
                            }
                        }
                    }
                    TokenKind::RightParen => {
                        return Result::Ok(lambda);
                    }
                    _ => {
                        match self.do_parse_form(lambda, next, |ast: &'a Ast<'a>| lambda.add_child(ast)) {
                            Ok(ast) => {},
                            Err(e) => {return Result::Err(e);}
                        }
                    }
                }
            })
        }
    }


    fn parse_literal(&self, token: Token<'a>) -> ParseResult<'a> {
        match token.kind() {
            TokenKind::Keyword => {
                Ok(Ast::new_keyword(self.zone_allocator, token, get_token_value!(self, token)))
            }
            TokenKind::Hex|
            TokenKind::Binary|
            TokenKind::Int => {
                match get_token_value!(self, token).parse::<i32>() {
                    Ok(i) => Ok(Ast::new_integer(self.zone_allocator, token, i)),
                    Err(_) => {
                        return Result::Err(ParseError::new("Invalid integer value.", token));
                    }
                }
            }
            TokenKind::Long|
            TokenKind::Float => {
                match get_token_value!(self, token).parse::<f64>() {
                    Ok(i) => Ok(Ast::new_double(self.zone_allocator, token, i)),
                    Err(_) => {
                        return Result::Err(ParseError::new("Invalid double value.", token));
                    }
                }
            }
            TokenKind::String => {
                Ok(Ast::new_string(self.zone_allocator, token, get_token_value!(self, token)))
            }
            TokenKind::Regexp => {
                Ok(Ast::new_regexp(self.zone_allocator, token, get_token_value!(self, token)))
            }
            TokenKind::Symbol => {
                return self.process_sym(token);
            }
            TokenKind::Boolean => {
                if get_token_value!(self, token) == "true" {
                    return Ok(Ast::new_boolean(self.zone_allocator, token, true));
                }
                return Ok(Ast::new_boolean(self.zone_allocator, token, false));
            }
            TokenKind::UnicodeChar => {
                let v = get_token_value!(self, token);
                match self.parse_unicode_escape_seq(v) {
                    Ok(i) => Ok(Ast::new_uchar(self.zone_allocator, token, i)),
                    Err(e) => Err(ParseError::new(e, token))
                }
            }
            TokenKind::Nil => {
                Ok(Ast::new_nil(self.zone_allocator, token))
            }
            TokenKind::QuoteRm => {
                let q = Ast::new_quote(self.zone_allocator, token);
                let next_token = self.scanner.scan();
                return match self.do_parse_form(q, next_token, |ast: &'a Ast<'a>| q.set_quote_expr(ast)) {
                    Err(e) => Err(e),
                    Ok(ast) => Ok(q)
                };
            }
            _ => {
                Err(ParseError::new("Invalid Token.", token))
            }
        }
    }


    fn process_sym(&self, token: Token<'a>) -> ParseResult<'a> {
        let v = get_token_value!(self, token);
        let sp: Vec<&'a str> = v.split('/').collect();
        if sp.len() == 1 {
            let sym = Ast::new_symbol(self.zone_allocator, token, v, SymbolMode::Unresolved);
            match self.find_scope(sym) {
                Some((d, s)) => {
                    match s.symbol_mode() {
                        SymbolMode::Parameter{index, depth} => {
                            sym.set_symbol_mode(SymbolMode::Parameter {index: index, depth: SymbolDepth::Depth(d)});
                        },
                        _ => {sym.set_symbol_mode(SymbolMode::Var(SymbolDepth::Depth(d)));}
                    }
                }
                None => {}
            }
            return Ok(sym);
        }

        let mr = Ast::new_module_reference(self.zone_allocator, token);
        for ns in sp {
            mr.add_child(Ast::new_symbol(self.zone_allocator, token, ns, SymbolMode::Var(SymbolDepth::Depth(0))));
        }
        return Ok(mr);
    }


    fn parse_unicode_escape_seq(&self, ue: &'a str) -> Result<i32, &'static str> {
        let mut result: i32 = 0;
        for u in ue.bytes() {
            let v = self.to_hex_value(u);
            if v < 0 {
                return Result::Err("Invalid unicode sequence.");
            }
            result += result * 16 + v;
        }
        Ok(result)
    }


    fn to_hex_value(&self, uchar: u8) -> i32 {
        if uchar >= '0' as u8 && uchar <= '9' as u8 {
            return (uchar as i32 - '0' as i32) as i32;
        } else if uchar >= 'a' as u8 && uchar <= 'f' as u8 {
            return (uchar as i32 - 'a' as i32 + 10) as i32;
        } else if uchar >= 'A' as u8 && uchar <= 'F' as u8 {
            return (uchar as i32 - 'A' as i32 + 10);
        }
        return -1;
    }


    fn find_scope(&self, ast: &'a Ast<'a>) -> Option<(u32, &'a Ast<'a>)> {
        self.scope_handler.scope().find(ast)
    }


    fn intern_scope(&self, ast: &'a Ast<'a>) {
        self.scope_handler.scope().intern(ast);
    }
}
