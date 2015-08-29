///! ///! 
///! The MIT License (MIT)
///! 
///! Copyright (c) 2013 Taketoshi Aono(brn)
///! 
///! Permission is hereby granted, free of charge, to any person obtaining a copy
///! of this software and associated documentation files (the "Software"), to deal
///! in the Software without restriction, including without limitation the rights
///! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
///! copies of the Software, and to permit persons to whom the Software is
///! furnished to do so, subject to the following conditions:
///! 
///! The above copyright notice and this permission notice shall be included in
///! all copies or substantial portions of the Software.
///! 
///! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
///! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
///! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
///! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
///! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
///! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
///! THE SOFTWARE.
///!
///! Ast defintions.
///!
///! Author Taketoshi Aono(brn) dobaw20@gmail.com


use std;
use std::cell::{Ref, RefCell, Cell};
use parser::token::{TokenKind, Token};
use parser::moduleinfo::ModuleInfo;
use parser::literal_buffer::{LiteralBuffer};
use internal::heap::zone::{ZoneAllocator, ZoneObject};


pub trait HasChild<'a> {
    fn children(&self) -> Ref<Vec<&'a Ast<'a>>>;
}


pub struct Module<'a> {
    module_info: &'a ModuleInfo,
    children: RefCell<Vec<&'a Ast<'a>>>,
}


impl<'a> Module<'a> {
    pub fn add_child(&self, child: &'a Ast<'a>) {
        self.children.borrow_mut().push(child);
    }
}


impl<'a> HasChild<'a> for Module<'a> {
    fn children(&self) -> Ref<Vec<&'a Ast<'a>>> {
        self.children.borrow()
    }
}


#[derive(Debug)]
pub enum FormKind {
    List,
    Lambda,
    Vector,
    Set,
    Map
}


pub struct Form<'a> {
    kind: FormKind,
    children: RefCell<Vec<&'a Ast<'a>>>,
    parent: Cell<Option<&'a Ast<'a>>>
}


impl<'a> HasChild<'a> for Form<'a> {
    fn children(&self) -> Ref<Vec<&'a Ast<'a>>> {
        self.children.borrow()
    }
}


impl<'a> Form<'a> {
    pub fn add_child(&self, child: &'a Ast<'a>) {
        self.children.borrow_mut().push(child);
    }


    pub fn children(&self) -> Ref<Vec<&'a Ast<'a>>>{
        self.children.borrow()
    }
}


pub struct Literal<'a> {
    token: Token<'a>,
    parent: Cell<Option<&'a Ast<'a>>>
}


impl<'a> Literal<'a> {
    pub fn kind(&self) -> TokenKind {
        self.token.kind()
    }


    pub fn token(&self) -> Token {
        self.token
    }
    

    pub fn is_keyword(&self) -> bool {
        self.token.kind() == TokenKind::Keyword
    }


    pub fn is_symbol(&self) -> bool {
        self.token.kind() == TokenKind::Symbol
    }


    pub fn is_string(&self) -> bool {
        self.token.kind() == TokenKind::String
    }


    pub fn is_boolean(&self) -> bool {
        self.token.kind() == TokenKind::Boolean
    }


    pub fn is_regexp(&self) -> bool {
        self.token.kind() == TokenKind::Regexp
    }


    pub fn is_nil(&self) -> bool {
        self.token.kind() == TokenKind::Nil
    }


    pub fn is_int(&self) -> bool {
        self.token.kind() == TokenKind::Int
    }
}


pub enum Ast<'a> {
    Module(Module<'a>),
    Form(Form<'a>),
    Literal(Literal<'a>)
}


impl<'a> ZoneObject<Ast<'a>> for Ast<'a> {}


impl<'a> Ast<'a> {
    pub fn new_module(za: &'a ZoneAllocator, module_info: &'a ModuleInfo) -> &'a Ast<'a> {
        za.alloc(Ast::Module(Module {
            module_info: module_info,
            children: RefCell::new(Vec::new())
        }))
    }
    
    pub fn new_form(za: &'a ZoneAllocator, kind: FormKind) -> &'a Ast<'a> {
        za.alloc(Ast::Form(Form {
            kind: kind,
            children: RefCell::new(Vec::new()),
            parent: Cell::new(Option::None)
        }))
    }

    pub fn new_literal(za: &'a ZoneAllocator, token: Token<'a>) -> &'a Ast<'a> {
        za.alloc(Ast::Literal(Literal {
            token: token,
            parent: Cell::new(Option::None)
        }))
    }


    pub fn set_parent(&self, parent: &'a Ast<'a>) {
        match *self {
            Ast::Form(ref f) => {
                f.parent.set(Option::Some(parent));
            }
            Ast::Literal(ref l) => {
                l.parent.set(Option::Some(parent));
            }
            _ => {}
        }
    }


    pub fn parent(&self) -> Option<&'a Ast<'a>> {
        match *self {
            Ast::Form(ref f) => f.parent.get(),
            Ast::Literal(ref l) => l.parent.get(),
            _ => Option::None
        }
    }


    pub fn to_string_tree(&self, literal_buffer: &'a LiteralBuffer) -> String {
        self.to_string_tree_helper("".to_string(), literal_buffer)
    }
    

    fn to_string_tree_helper(&self, indent: String, literal_buffer: &'a LiteralBuffer) -> String {
        match *self {
            Ast::Form(ref f) => {
                let mut base = format!("{}{:?}", indent, f.kind);
                for child in f.children().iter() {
                    base = format!("{}\n{}", base, child.to_string_tree_helper(format!("  {}", indent), literal_buffer));
                }
                base
            }
            Ast::Module(ref f) => {
                let mut base = format!("{}Module", indent);
                for child in f.children().iter() {
                    base = format!("{}\n{}", base, child.to_string_tree_helper(format!("  {}", indent), literal_buffer));
                }
                base
            }
            Ast::Literal(ref l) => {
                format!("{}Literal({}, '{}')", indent, l.token(), literal_buffer.find(l.token().value()))
            }
        }
    }
}
