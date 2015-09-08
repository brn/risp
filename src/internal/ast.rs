///!
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
use std::cell::{Ref, RefCell, RefMut, Cell};
use std::collections::{HashMap};
use parser::token::{TokenKind, Token};
use parser::moduleinfo::ModuleInfo;
use parser::literal_buffer::{LiteralBuffer};
use internal::heap::zone::{ZoneAllocator, ZoneObject};


pub enum Ast<'a> {
    Module(Module<'a>),
    Map(Map<'a>),
    Set(Set<'a>),
    List(List<'a>),
    Tag(Tag<'a>),
    If(If<'a>),
    Quote(Quote<'a>),
    Def(Def<'a>),
    Vector(Vector<'a>),
    Let(Let<'a>),
    Lambda(Lambda<'a>),
    DefMacro(DefMacro<'a>),
    LambdaSugar(LambdaSugar<'a>),
    Integer(Integer<'a>),
    Double(Double<'a>),
    String(String<'a>),
    UChar(UChar<'a>),
    Symbol(Symbol<'a>),
    ModuleReference(ModuleReference<'a>),
    Keyword(Keyword<'a>),
    Boolean(Boolean<'a>),
    RegExp(RegExp<'a>),
    LambdaParam(LambdaParam<'a>),
    Nil(Nil<'a>)
}


pub trait AstVisitor<'a, T> {
    fn visit_module(&self, ast: &'a Ast<'a>) -> T;

    fn visit_tag(&self, ast: &'a Ast<'a>) -> T;

    fn visit_map(&self, ast: &'a Ast<'a>) -> T;

    fn visit_set(&self, ast: &'a Ast<'a>) -> T;

    fn visit_let(&self, ast: &'a Ast<'a>) -> T;
    
    fn visit_list(&self, ast: &'a Ast<'a>) -> T;

    fn visit_quote(&self, ast: &'a Ast<'a>) -> T;

    fn visit_if(&self, ast: &'a Ast<'a>) -> T;

    fn visit_def(&self, ast: &'a Ast<'a>) -> T;

    fn visit_vector(&self, ast: &'a Ast<'a>) -> T;

    fn visit_lambda(&self, ast: &'a Ast<'a>) -> T;

    fn visit_defmacro(&self, ast: &'a Ast<'a>) -> T;

    fn visit_module_ref(&self, ast: &'a Ast<'a>) -> T;

    fn visit_lambda_sugar(&self, ast: &'a Ast<'a>) -> T;

    fn visit_integer(&self, ast: &'a Ast<'a>) -> T;

    fn visit_double(&self, ast: &'a Ast<'a>) -> T;

    fn visit_string(&self, ast: &'a Ast<'a>) -> T;

    fn visit_uchar(&self, ast: &'a Ast<'a>) -> T;

    fn visit_symbol(&self, ast: &'a Ast<'a>) -> T;

    fn visit_keyword(&self, ast: &'a Ast<'a>) -> T;

    fn visit_boolean(&self, ast: &'a Ast<'a>) -> T;

    fn visit_regexp(&self, ast: &'a Ast<'a>) -> T;

    fn visit_lambda_param(&self, ast: &'a Ast<'a>) -> T;

    fn visit_nil(&self, ast: &'a Ast<'a>) -> T;
}


pub trait Dispatcher<'a, 'v> {
    fn visit<T>(&'a self, visitor: &'v AstVisitor<'a, T>) -> T;
}


impl<'a, 'v> Dispatcher<'a, 'v> for Ast<'a> {
    fn visit<T>(&'a self, visitor: &'v AstVisitor<'a, T>) -> T{
        match self {
            &Ast::Module(_) => visitor.visit_module(self),
            &Ast::Map(_) => visitor.visit_map(self),
            &Ast::Set(_) => visitor.visit_set(self),
            &Ast::List(_) => visitor.visit_list(self),
            &Ast::Quote(_) => visitor.visit_quote(self),
            &Ast::Let(_) => visitor.visit_let(self),
            &Ast::If(_) => visitor.visit_if(self),
            &Ast::Def(_) => visitor.visit_def(self),
            &Ast::DefMacro(_) => visitor.visit_defmacro(self),
            &Ast::Tag(_) => visitor.visit_tag(self),
            &Ast::Vector(_) => visitor.visit_vector(self),
            &Ast::Lambda(_) => visitor.visit_lambda(self),
            &Ast::LambdaSugar(_) => visitor.visit_lambda_sugar(self),
            &Ast::Integer(_) => visitor.visit_integer(self),
            &Ast::Double(_) => visitor.visit_double(self),
            &Ast::String(_) => visitor.visit_string(self),
            &Ast::UChar(_) => visitor.visit_uchar(self),
            &Ast::Symbol(_) => visitor.visit_symbol(self),
            &Ast::ModuleReference(_) => visitor.visit_module_ref(self),
            &Ast::Keyword(_) => visitor.visit_keyword(self),
            &Ast::Boolean(_) => visitor.visit_boolean(self),
            &Ast::RegExp(_) => visitor.visit_regexp(self),
            &Ast::LambdaParam(_) => visitor.visit_lambda_param(self),
            &Ast::Nil(_) => visitor.visit_nil(self)
        }
    }
}


macro_rules! ast_name {
    ($ast:expr) => {
        match $ast {
            &Ast::Module(_) => "Module",
            &Ast::Map(_) => "Map",
            &Ast::Set(_) => "Set",
            &Ast::List(_) => "List",
            &Ast::Quote(_) => "Quote",
            &Ast::If(_) => "If",
            &Ast::Let(_) => "Let",
            &Ast::Def(_) => "Def",
            &Ast::Tag(_) => "Tag",
            &Ast::Vector(_) => "Vector",
            &Ast::Lambda(_) => "Lambda",
            &Ast::DefMacro(_) => "DefMacro",
            &Ast::LambdaSugar(_) => "Lambda",
            &Ast::Integer(_) => "Integer",
            &Ast::Double(_) => "Double",
            &Ast::String(_) => "String",
            &Ast::UChar(_) => "UChar",
            &Ast::Symbol(_) => "Symbol",
            &Ast::ModuleReference(_) => "ModuleReference",
            &Ast::Keyword(_) => "Keyword",
            &Ast::Boolean(_) => "Boolean",
            &Ast::RegExp(_) => "RegExp",
            &Ast::LambdaParam(_) => "LambdaParam",
            &Ast::Nil(_) => "Nil"
        }
    }
}


pub trait HasChildren<'a> {
    fn children(&self) -> Ref<Vec<&'a Ast<'a>>>;

    fn children_mut(&self) -> RefMut<Vec<&'a Ast<'a>>>;

    fn add_child(&self, child: &'a Ast<'a>);
}


macro_rules! has_children_impl {
    ($t:ty) => {
        impl<'a> HasChildren<'a> for $t {
            fn children(&self) -> Ref<Vec<&'a Ast<'a>>> {
                self.children.borrow()
            }


            fn children_mut(&self) -> RefMut<Vec<&'a Ast<'a>>> {
                self.children.borrow_mut()
            }


            fn add_child(&self, child: &'a Ast<'a>) {
                self.children.borrow_mut().push(child);
            }
        }
    }
}


pub trait HasParent<'a> {
    fn parent(&self) -> Option<&'a Ast<'a>>;

    fn set_parent(&self, parent: &'a Ast<'a>);
}


macro_rules! has_parent_impl {
    ($t:ty) => {
        impl<'a> HasParent<'a> for $t {
            fn parent(&self) -> Option<&'a Ast<'a>> {
                self.parent.get()
            }

            fn set_parent(&self, parent: &'a Ast<'a>) {
                self.parent.set(Some(parent));
            }
        }
    }
}


pub trait HasToken<'a> {
    fn token(&self) -> Token<'a>;
}


macro_rules! has_token_impl {
    ($t:ty) => {
        impl<'a> HasToken<'a> for $t {
            fn token(&self) -> Token<'a> {
                self.token
            }
        }
    }
}


macro_rules! generic_impl {
    ($t:ty) => {
        has_children_impl!($t);
        has_parent_impl!($t);
        has_token_impl!($t);
    }
}


macro_rules! literal_impl {
    ($t:ty) => {
        has_parent_impl!($t);
        has_token_impl!($t);
    }
}


macro_rules! literal_unwrap {
    ($ast:expr, $t:ty) => {
        match $ast {
            &Ast::Tag(ref a) => Some(a as &$t),
            &Ast::Map(ref a) => Some(a as &$t),
            &Ast::Set(ref a) => Some(a as &$t),
            &Ast::List(ref a) => Some(a as &$t),
            &Ast::Quote(ref a) => Some(a as &$t),
            &Ast::Let(ref a) => Some(a as &$t),
            &Ast::If(ref a) => Some(a as &$t),
            &Ast::Def(ref a) => Some(a as &$t),
            &Ast::Vector(ref a) => Some(a as &$t),
            &Ast::ModuleReference(ref a) => Some(a as &$t),
            &Ast::Lambda(ref a) => Some(a as &$t),
            &Ast::DefMacro(ref a) => Some(a as &$t),
            &Ast::LambdaSugar(ref a) => Some(a as &$t),
            &Ast::Integer(ref a) => Some(a as &$t),
            &Ast::Double(ref a) => Some(a as &$t),
            &Ast::String(ref a) => Some(a as &$t),
            &Ast::Symbol(ref a) => Some(a as &$t),
            &Ast::UChar(ref a) => Some(a as &$t),
            &Ast::Keyword(ref a) => Some(a as &$t),
            &Ast::Boolean(ref a) => Some(a as &$t),
            &Ast::RegExp(ref a) => Some(a as &$t),
            &Ast::LambdaParam(ref a) => Some(a as &$t),
            &Ast::Nil(ref a) => Some(a as &$t),
            &Ast::Module(ref m) => None
        }
    }
}

macro_rules! unwrap_has_parent {
    ($ast:expr) => {
        literal_unwrap!($ast, HasParent)
    }
}

macro_rules! unwrap_has_token {
    ($ast:expr) => {
        literal_unwrap!($ast, HasToken)
    }
}



macro_rules! unwrap_has_children {
    ($ast:expr) => {
        match $ast {
            &Ast::Module(ref a) => Option::Some(a as &HasChildren),
            &Ast::Map(ref a) => Option::Some(a as &HasChildren),
            &Ast::Set(ref a) => Option::Some(a as &HasChildren),
            &Ast::List(ref a) => Option::Some(a as &HasChildren),
            &Ast::Tag(ref a) => Option::Some(a as &HasChildren),
            &Ast::LambdaSugar(ref a) => Option::Some(a as &HasChildren),
            &Ast::Vector(ref a) => Option::Some(a as &HasChildren),
            &Ast::ModuleReference(ref a) => Option::Some(a as &HasChildren),
            _ => Option::None
        }
    }
}


pub struct Scope<'a> {
    map: RefCell<HashMap<i64, &'a Ast<'a>>>,
    parent: Cell<Option<&'a Scope<'a>>>,
    id: i32,
    origin: Cell<Option<&'a Ast<'a>>>,
    depth: Cell<u32>
}


impl<'a> ZoneObject<Scope<'a>> for Scope<'a> {}


impl<'a> std::fmt::Display for Scope<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Scope(id = {}, depth = {})", self.id, self.depth.get())
    }
}


impl<'a> Scope<'a> {
    pub fn new(za: &'a ZoneAllocator, id: i32) -> &'a Scope<'a> {
        za.alloc(Scope {
            map: RefCell::new(HashMap::new()),
            parent: Cell::new(None),
            id: id,
            origin: Cell::new(None),
            depth: Cell::new(0)
        })
    }


    pub fn new_with_parent(za: &'a ZoneAllocator, parent: &'a Scope<'a>, id: i32) -> &'a Scope<'a> {
        za.alloc(Scope {
            map: RefCell::new(HashMap::new()),
            parent: Cell::new(None),
            id: id,
            origin: Cell::new(None),
            depth: Cell::new(parent.depth.get() + 1)
        })
    }


    pub fn map(&self) -> Ref<HashMap<i64, &'a Ast<'a>>> {
        self.map.borrow()
    }
    

    pub fn id(&self) -> i32 {
        self.id
    }


    pub fn depth(&self) -> u32 {
        self.depth.get()
    }
    

    pub fn parent(&self) -> Option<&'a Scope<'a>> {
        self.parent.get()
    }


    pub fn set_origin(&self, ast: &'a Ast<'a>) {
        self.origin.set(Some(ast));
    }


    pub fn origin(&self) -> Option<&'a Ast<'a>> {
        self.origin.get()
    }


    pub fn set_parent(&self, parent: &'a Scope<'a>) {
        self.parent.set(Some(parent));
        self.depth.set(parent.depth.get() + 1);
    }


    pub fn intern(&self, symbol: &'a Ast<'a>) {
        match symbol {
            &Ast::Symbol(ref s) => {
                self.map.borrow_mut().insert(s.token().value(), symbol);
            }
            _ => {panic!("Scope::intern argument must be a Symbol.");}
        }
    }


    pub fn find(&self, symbol: &'a Ast<'a>) -> Option<(u32, &'a Ast<'a>)> {
        match symbol {
            &Ast::Symbol(ref s) => {
                let mut scope = self;
                let mut depth = 0;
                loop {
                    let map = scope.map.borrow();
                    let mut v = map.get(&s.token().value());
                    match v {
                        Some(ast) => {return Some((depth, *ast));}
                        None => {
                            match scope.parent.get() {
                                Some(p) => {
                                    scope = p;
                                },
                                None => {
                                    return None;
                                }
                            }
                        }
                    }
                    depth += 1;
                }
            }
            _ => {panic!("Scope::find argument must be a Symbol.");}
        }
    }
}


pub struct Module<'a> {
    module_info: &'a ModuleInfo,
    scope: &'a Scope<'a>,
    children: RefCell<Vec<&'a Ast<'a>>>,
}
has_children_impl!(Module<'a>);


pub struct List<'a> {
    token: Token<'a>,
    children: RefCell<Vec<&'a Ast<'a>>>,
    parent: Cell<Option<&'a Ast<'a>>>
}
generic_impl!(List<'a>);


pub struct If<'a> {
    token: Token<'a>,
    cond: Cell<Option<&'a Ast<'a>>>,
    then_body: Cell<Option<&'a Ast<'a>>>,
    else_body: Cell<Option<&'a Ast<'a>>>,
    parent: Cell<Option<&'a Ast<'a>>>
}
literal_impl!(If<'a>);


pub struct Def<'a> {
    token: Token<'a>,
    name: Cell<Option<&'a Ast<'a>>>,
    expr: Cell<Option<&'a Ast<'a>>>,
    parent: Cell<Option<&'a Ast<'a>>>
}
literal_impl!(Def<'a>);


pub struct Quote<'a> {
    token: Token<'a>,
    expr: Cell<Option<&'a Ast<'a>>>,
    parent: Cell<Option<&'a Ast<'a>>>
}
literal_impl!(Quote<'a>);


pub struct Tag<'a> {
    token: Token<'a>,
    children: RefCell<Vec<&'a Ast<'a>>>,
    parent: Cell<Option<&'a Ast<'a>>>
}
generic_impl!(Tag<'a>);


pub struct Map<'a> {
    token: Token<'a>,
    children: RefCell<Vec<&'a Ast<'a>>>,
    parent: Cell<Option<&'a Ast<'a>>>
}
generic_impl!(Map<'a>);


pub struct ModuleReference<'a> {
    token: Token<'a>,
    children: RefCell<Vec<&'a Ast<'a>>>,
    parent: Cell<Option<&'a Ast<'a>>>
}
generic_impl!(ModuleReference<'a>);


pub struct Set<'a> {
    token: Token<'a>,
    children: RefCell<Vec<&'a Ast<'a>>>,
    parent: Cell<Option<&'a Ast<'a>>>
}
generic_impl!(Set<'a>);


pub struct Vector<'a> {
    token: Token<'a>,
    children: RefCell<Vec<&'a Ast<'a>>>,
    parent: Cell<Option<&'a Ast<'a>>>
}
generic_impl!(Vector<'a>);


pub struct Let<'a> {
    token: Token<'a>,
    body: RefCell<Vec<&'a Ast<'a>>>,
    bindings: RefCell<Vec<(&'a Ast<'a>, &'a Ast<'a>)>>,
    scope: &'a Scope<'a>,
    parent: Cell<Option<&'a Ast<'a>>>
}
literal_impl!(Let<'a>);


pub struct Lambda<'a> {
    token: Token<'a>,
    arguments: RefCell<Vec<&'a Ast<'a>>>,
    body: RefCell<Vec<&'a Ast<'a>>>,
    scope: &'a Scope<'a>,
    parent: Cell<Option<&'a Ast<'a>>>
}
literal_impl!(Lambda<'a>);


pub struct DefMacro<'a> {
    token: Token<'a>,
    name: &'a Ast<'a>,
    arguments: RefCell<Vec<&'a Ast<'a>>>,
    body: RefCell<Vec<&'a Ast<'a>>>,
    scope: &'a Scope<'a>,
    parent: Cell<Option<&'a Ast<'a>>>
}
literal_impl!(DefMacro<'a>);


pub struct LambdaSugar<'a> {
    token: Token<'a>,
    children: RefCell<Vec<&'a Ast<'a>>>,
    parent: Cell<Option<&'a Ast<'a>>>
}
generic_impl!(LambdaSugar<'a>);


pub struct Integer<'a> {
    token: Token<'a>,
    value: i32,
    parent: Cell<Option<&'a Ast<'a>>>
}
literal_impl!(Integer<'a>);



pub struct Double<'a> {
    token: Token<'a>,
    value: f64,
    parent: Cell<Option<&'a Ast<'a>>>
}
literal_impl!(Double<'a>);



pub struct String<'a> {
    token: Token<'a>,
    value: &'a str,
    parent: Cell<Option<&'a Ast<'a>>>
}
literal_impl!(String<'a>);



pub struct UChar<'a> {
    token: Token<'a>,
    value: i32,
    parent: Cell<Option<&'a Ast<'a>>>
}
literal_impl!(UChar<'a>);


#[derive(Copy, Clone)]
pub enum SymbolDepth {
    Origin,
    Depth(u32)
}


#[derive(Copy, Clone)]
pub enum SymbolMode {
    Unresolved,
    Var(SymbolDepth),
    Parameter {
        index: i32,
        depth: SymbolDepth
    }
}


pub struct Symbol<'a> {
    token: Token<'a>,
    value: &'a str,
    bound: Cell<Option<&'a Ast<'a>>>,
    mode: Cell<SymbolMode>,
    parent: Cell<Option<&'a Ast<'a>>>
}
literal_impl!(Symbol<'a>);


pub struct Keyword<'a> {
    token: Token<'a>,
    value: &'a str,
    parent: Cell<Option<&'a Ast<'a>>>
}
literal_impl!(Keyword<'a>);



pub struct Boolean<'a> {
    token: Token<'a>,
    value: bool,
    parent: Cell<Option<&'a Ast<'a>>>
}
literal_impl!(Boolean<'a>);


pub struct RegExp<'a> {
    token: Token<'a>,
    value: &'a str,
    parent: Cell<Option<&'a Ast<'a>>>
}
literal_impl!(RegExp<'a>);


pub struct LambdaParam<'a> {
    token: Token<'a>,
    value: i32,
    parent: Cell<Option<&'a Ast<'a>>>
}
literal_impl!(LambdaParam<'a>);


pub struct Nil<'a> {
    token: Token<'a>,
    parent: Cell<Option<&'a Ast<'a>>>
}
literal_impl!(Nil<'a>);


impl<'a> ZoneObject<Ast<'a>> for Ast<'a> {}


impl<'a> Ast<'a> {
    pub fn new_module(za: &'a ZoneAllocator, module_info: &'a ModuleInfo, scope: &'a Scope<'a>) -> &'a Ast<'a> {
        za.alloc(Ast::Module(Module {
            module_info: module_info,
            scope: scope,
            children: RefCell::new(Vec::new())
        }))
    }


    pub fn new_list(za: &'a ZoneAllocator, token: Token<'a>) -> &'a Ast<'a> {
        za.alloc(Ast::List(List {
            token: token,
            children: RefCell::new(Vec::new()),
            parent: Cell::new(Option::None)
        }))
    }


    pub fn new_if(za: &'a ZoneAllocator, token: Token<'a>) -> &'a Ast<'a> {
        za.alloc(Ast::If(If {
            token: token,
            cond: Cell::new(None),
            then_body: Cell::new(None),
            else_body: Cell::new(None),
            parent: Cell::new(None)
        }))
    }


    pub fn new_quote(za: &'a ZoneAllocator, token: Token<'a>) -> &'a Ast<'a> {
        za.alloc(Ast::Quote(Quote {
            token: token,
            expr: Cell::new(Option::None),
            parent: Cell::new(Option::None)
        }))
    }
    

    pub fn new_def(za: &'a ZoneAllocator, token: Token<'a>) -> &'a Ast<'a> {
        za.alloc(Ast::Def(Def {
            token: token,
            name: Cell::new(None),
            expr: Cell::new(None),
            parent: Cell::new(Option::None)
        }))
    }


    pub fn new_tag(za: &'a ZoneAllocator, token: Token<'a>) -> &'a Ast<'a> {
        za.alloc(Ast::Tag(Tag {
            token: token,
            children: RefCell::new(Vec::new()),
            parent: Cell::new(Option::None)
        }))
    }


    pub fn new_map(za: &'a ZoneAllocator, token: Token<'a>) -> &'a Ast<'a> {
        za.alloc(Ast::Map(Map {
            token: token,
            children: RefCell::new(Vec::new()),
            parent: Cell::new(Option::None)
        }))
    }


    pub fn new_set(za: &'a ZoneAllocator, token: Token<'a>) -> &'a Ast<'a> {
        za.alloc(Ast::Set(Set {
            token: token,
            children: RefCell::new(Vec::new()),
            parent: Cell::new(Option::None)
        }))
    }


    pub fn new_vector(za: &'a ZoneAllocator, token: Token<'a>) -> &'a Ast<'a> {
        za.alloc(Ast::Vector(Vector {
            token: token,
            children: RefCell::new(Vec::new()),
            parent: Cell::new(Option::None)
        }))
    }


    pub fn new_module_reference(za: &'a ZoneAllocator, token: Token<'a>) -> &'a Ast<'a> {
        za.alloc(Ast::ModuleReference(ModuleReference {
            token: token,
            children: RefCell::new(Vec::new()),
            parent: Cell::new(None)
        }))
    }


    pub fn new_let(za: &'a ZoneAllocator, token: Token<'a>, scope: &'a Scope<'a>) -> &'a Ast<'a> {
        za.alloc(Ast::Let(Let {
            token: token,
            bindings: RefCell::new(Vec::new()),
            body: RefCell::new(Vec::new()),
            scope: scope,
            parent: Cell::new(Option::None)
        }))
    }


    pub fn new_lambda(za: &'a ZoneAllocator, token: Token<'a>, scope: &'a Scope<'a>) -> &'a Ast<'a> {
        za.alloc(Ast::Lambda(Lambda {
            token: token,
            arguments: RefCell::new(Vec::new()),
            body: RefCell::new(Vec::new()),
            scope: scope,
            parent: Cell::new(Option::None)
        }))
    }


    pub fn new_defmacro(za: &'a ZoneAllocator, token: Token<'a>, name: &'a Ast<'a>, scope: &'a Scope<'a>) -> &'a Ast<'a> {
        za.alloc(Ast::DefMacro(DefMacro {
            token: token,
            name: name,
            arguments: RefCell::new(Vec::new()),
            body: RefCell::new(Vec::new()),
            scope: scope,
            parent: Cell::new(Option::None)
        }))
    }


    pub fn new_lambda_sugar(za: &'a ZoneAllocator, token: Token<'a>) -> &'a Ast<'a> {
        za.alloc(Ast::LambdaSugar(LambdaSugar {
            token: token,
            children: RefCell::new(Vec::new()),
            parent: Cell::new(Option::None)
        }))
    }
    

    pub fn new_integer(za: &'a ZoneAllocator, token: Token<'a>, value: i32) -> &'a Ast<'a> {
        za.alloc(Ast::Integer(Integer {
            token: token,
            value: value,
            parent: Cell::new(Option::None)
        }))
    }


    pub fn new_double(za: &'a ZoneAllocator, token: Token<'a>, value: f64) -> &'a Ast<'a> {
        za.alloc(Ast::Double(Double {
            token: token,
            value: value,
            parent: Cell::new(Option::None)
        }))
    }


    pub fn new_string(za: &'a ZoneAllocator, token: Token<'a>, value: &'a str) -> &'a Ast<'a> {
        za.alloc(Ast::String(String {
            token: token,
            value: value,
            parent: Cell::new(Option::None)
        }))
    }


    pub fn new_symbol(za: &'a ZoneAllocator, token: Token<'a>, value: &'a str, mode: SymbolMode) -> &'a Ast<'a> {
        za.alloc(Ast::Symbol(Symbol {
            token: token,
            value: value,
            mode: Cell::new(mode),
            bound: Cell::new(None),
            parent: Cell::new(None)
        }))
    }


    pub fn new_uchar(za: &'a ZoneAllocator, token: Token<'a>, value: i32) -> &'a Ast<'a> {
        za.alloc(Ast::UChar(UChar {
            token: token,
            value: value,
            parent: Cell::new(Option::None)
        }))
    }


    pub fn new_keyword(za: &'a ZoneAllocator, token: Token<'a>, value: &'a str) -> &'a Ast<'a> {
        za.alloc(Ast::Keyword(Keyword {
            token: token,
            value: value,
            parent: Cell::new(Option::None)
        }))
    }


    pub fn new_boolean(za: &'a ZoneAllocator, token: Token<'a>, value: bool) -> &'a Ast<'a> {
        za.alloc(Ast::Boolean(Boolean {
            token: token,
            value: value,
            parent: Cell::new(Option::None)
        }))
    }


    pub fn new_regexp(za: &'a ZoneAllocator, token: Token<'a>, value: &'a str) -> &'a Ast<'a> {
        za.alloc(Ast::RegExp(RegExp {
            token: token,
            value: value,
            parent: Cell::new(Option::None)
        }))
    }


    pub fn new_lambda_param(za: &'a ZoneAllocator, token: Token<'a>, value: i32) -> &'a Ast<'a> {
        za.alloc(Ast::LambdaParam(LambdaParam {
            token: token,
            value: value,
            parent: Cell::new(Option::None)
        }))
    }

    
    pub fn new_nil(za: &'a ZoneAllocator, token: Token<'a>) -> &'a Ast<'a> {
        za.alloc(Ast::Nil(Nil {
            token: token,
            parent: Cell::new(Option::None)
        }))
    }


    pub fn set_symbol_mode(&self, mode: SymbolMode) {
        match self {
            &Ast::Symbol(ref s) => {
                s.mode.set(mode);
            }
            _ => {panic!("set_symbol_mode called to non symbol ast.");}
        }
    }


    pub fn symbol_mode(&self) -> SymbolMode {
        match self {
            &Ast::Symbol(ref s) => {
                s.mode.get()
            }
            _ => {panic!("symbol_mode called to non symbol ast.");}
        }
    }


    pub fn bind_to_symbol(&self, val: &'a Ast<'a>) {
        match self {
            &Ast::Symbol(ref s) => {
                s.bound.set(Some(val))
            }
            _ => {panic!("Ast::bind_to_symbol called to non symbol ast.");}
        }
    }


    pub fn symbol_bounded_value(&self) -> Option<&'a Ast<'a>> {
        match self {
            &Ast::Symbol(ref s) => {
                s.bound.get()
            }
            _ => {panic!("Ast::symbol_bounded_value called to non symbol ast.");}
        }
    }


    pub fn set_parent_scope(&self, scope: &'a Scope<'a>) {
        match self {
            &Ast::Lambda(ref l) => {
                l.scope.parent.set(Some(scope));
            },
            &Ast::DefMacro(ref d) => {
                d.scope.parent.set(Some(scope));
            }
            &Ast::Let(ref l) => {
                l.scope.parent.set(Some(scope));
            }
            _ => {panic!("set_parent_scope called to non scoped ast.");}
        }
    }


    pub fn parent_scope(&self) -> Option<&'a Scope<'a>> {
        match self {
            &Ast::Lambda(ref l) => {
                l.scope.parent.get()
            },
            &Ast::DefMacro(ref d) => {
                d.scope.parent.get()
            }
            &Ast::Let(ref l) => {
                l.scope.parent.get()
            }
            _ => {panic!("parent_scope called to non scoped ast.");}
        }
    }


    pub fn scope(&self) -> Option<&'a Scope<'a>> {
        match self {
            &Ast::Lambda(ref l) => Some(l.scope),
            &Ast::Module(ref m) => Some(m.scope),
            &Ast::Let(ref l) => Some(l.scope),
            _ => {panic!("Ast::scope called to not lambda or module ast.");}
        }
    }


    pub fn set_def_name(&self, name: &'a Ast<'a>) {
        match self {
            &Ast::Def(ref d) => {
                d.name.set(Some(name));
            }
            _ => {}
        }
    }


    pub fn def_name(&self) -> Option<&'a Ast<'a>> {
        match self {
            &Ast::Def(ref d) => {
                d.name.get()
            }
            _ => None
        }
    }


    pub fn set_def_expr(&self, expr: &'a Ast<'a>) {
        match self {
            &Ast::Def(ref d) => {
                d.expr.set(Some(expr));
            }
            _ => {}
        }
    }


    pub fn def_expr(&self) -> Option<&'a Ast<'a>> {
        match self {
            &Ast::Def(ref d) => {
                d.expr.get()
            }
            _ => None
        }
    }
    

    pub fn add_lambda_arg(&self, arg: &'a Ast<'a>) {
        match self {
            &Ast::Lambda(ref l) => {
                l.arguments.borrow_mut().push(arg);
            },
            _ => {panic!("add_lambda_arg called to non lambda ast.");}
        }
    }


    pub fn add_lambda_body(&self, body: &'a Ast<'a>) {
        match self {
            &Ast::Lambda(ref l) => {
                l.body.borrow_mut().push(body);
            },
            _ => {panic!("add_lambda_body called to non lambda ast.");}
        }
    }


    pub fn macro_name(&self) -> &'a Ast<'a> {
        match self {
            &Ast::DefMacro(ref d) => {
                d.name
            },
            _ => {panic!("macro_name called to non defmacro ast.");}
        }
    }


    pub fn add_macro_arg(&self, arg: &'a Ast<'a>) {
        match self {
            &Ast::DefMacro(ref d) => {
                d.arguments.borrow_mut().push(arg);
            },
            _ => {panic!("add_macro_arg called to non defmacro ast.");}
        }
    }


    pub fn add_macro_body(&self, body: &'a Ast<'a>) {
        match self {
            &Ast::DefMacro(ref d) => {
                d.body.borrow_mut().push(body);
            },
            _ => {panic!("add_macro_body called to non defmacro ast.");}
        }
    }


    pub fn add_let_binding(&self, binding: (&'a Ast<'a>, &'a Ast<'a>)) {
        match self {
            &Ast::Let(ref d) => {
                d.bindings.borrow_mut().push(binding);
            },
            _ => {panic!("add_let_binding called to non let ast.");}
        }
    }


    pub fn add_let_body(&self, body: &'a Ast<'a>) {
        match self {
            &Ast::Let(ref d) => {
                d.body.borrow_mut().push(body);
            },
            _ => {panic!("add_let_body called to non let ast.");}
        }
    }


    pub fn let_body(&self) -> Ref<Vec<&'a Ast<'a>>> {
        match self {
            &Ast::Let(ref d) => {
                d.body.borrow()
            },
            _ => {panic!("let_body called to non let ast.");}
        }
    }


    pub fn set_quote_expr(&self, expr: &'a Ast<'a>) {
        match self {
            &Ast::Quote(ref i) => {
                i.expr.set(Some(expr));
            },
            _ => {panic!("set_quote_expr called to non quote ast.");}
        }
    }


    pub fn quote_expr(&self) -> Option<&'a Ast<'a>> {
        match self {
            &Ast::Quote(ref i) => {
                i.expr.get()
            },
            _ => {panic!("quote_expr called to non quote ast.");}
        }
    }


    pub fn set_cond(&self, cond: &'a Ast<'a>) {
        match self {
            &Ast::If(ref i) => {
                i.cond.set(Some(cond));
            },
            _ => {panic!("set_cond called to non if ast.");}
        }
    }


    pub fn set_then_body(&self, body: &'a Ast<'a>) {
        match self {
            &Ast::If(ref i) => {
                i.then_body.set(Some(body));
            },
            _ => {panic!("set_then_body called to non if ast.");}
        }
    }


    pub fn set_else_body(&self, body: &'a Ast<'a>) {
        match self {
            &Ast::If(ref i) => {
                i.else_body.set(Some(body));
            },
            _ => {panic!("set_else_body called to non if ast.");}
        }
    }


    pub fn then_body(&self) -> Option<&'a Ast<'a>> {
        match self {
            &Ast::If(ref i) => {
                i.then_body.get()
            },
            _ => {panic!("then_body called to non if ast.");}
        }
    }


    pub fn else_body(&self) -> Option<&'a Ast<'a>> {
        match self {
            &Ast::If(ref i) => {
                i.else_body.get()
            },
            _ => {panic!("else_body called to non if ast.");}
        }
    }


    pub fn lambda_arguments(&self, arg: &'a Ast<'a>) -> Option<Ref<Vec<&'a Ast<'a>>>> {
        match self {
            &Ast::Lambda(ref l) => {
                Option::Some(l.arguments.borrow())
            },
            _ => Option::None
        }
    }


    pub fn lambda_body(&self, body: &'a Ast<'a>) -> Option<Ref<Vec<&'a Ast<'a>>>> {
        match self {
            &Ast::Lambda(ref l) => {
                Option::Some(l.body.borrow())
            },
            _ => Option::None
        }
    }


    pub fn add_child(&self, child: &'a Ast<'a>) {
        match unwrap_has_children!(self) {
            Some(a) => {a.add_child(child)}
            None => {panic!("Specified ast can not have children.")}
        }
    }

    
    pub fn children_mut(&self, child: &'a Ast<'a>) -> Option<RefMut<Vec<&'a Ast<'a>>>> {
        match unwrap_has_children!(self) {
            Some(c) => Option::Some(c.children_mut()),
            None => Option::None
        }
    }


    pub fn children(&self) -> Option<Ref<Vec<&'a Ast<'a>>>> {
        match unwrap_has_children!(self) {
            Some(c) => Option::Some(c.children()),
            None => Option::None
        }
    }


    pub fn set_parent(&self, parent: &'a Ast<'a>) {
        match unwrap_has_parent!(self) {
            Some(a) => a.set_parent(parent),
            None => {}
        }
    }


    pub fn parent(&self) -> Option<&'a Ast<'a>> {
        match unwrap_has_parent!(self) {
            Some(a) => a.parent(),
            None => None
        }
    }


    pub fn token(&self) -> Option<Token<'a>> {
        match unwrap_has_token!(self) {
            Some(a) => Some(a.token()),
            None => None
        }
    }
    

    pub fn string_value(&self) -> Option<&'a str> {
        match self {
            &Ast::String(ref a) => Option::Some(a.value),
            &Ast::RegExp(ref a) => Option::Some(a.value),
            &Ast::Keyword(ref a) => Option::Some(a.value),
            _ => Option::None
        }
    }


    pub fn int_value(&self) -> Option<i32> {
        match self {
            &Ast::Integer(ref i) => Option::Some(i.value),
            _ => Option::None
        }
    }


    pub fn double_value(&self) -> Option<f64> {
        match self {
            &Ast::Double(ref d) => Option::Some(d.value),
            _ => Option::None
        }
    }


    pub fn boolean_value(&self) -> Option<bool> {
        match self {
            &Ast::Boolean(ref b) => Option::Some(b.value),
            _ => Option::None
        }
    }


    pub fn is_nil(&self) -> bool {
        match self {
            &Ast::Nil(_) => true,
            _ => false
        }
    }


    pub fn to_string_tree(&self) -> std::string::String {
        self.to_string_tree_helper("".to_string())
    }
    

    fn to_string_tree_helper(&self, indent: std::string::String) -> std::string::String {
        match unwrap_has_children!(self) {
            Some(c) => {
                let mut base = match self {
                    &Ast::Module(ref m) => format!("{}{}({})", indent, ast_name!(self), m.scope),
                    _ => format!("{}{}", indent, ast_name!(self))
                };
                for child in c.children().iter() {
                    base = format!("{}\n{}", base, child.to_string_tree_helper(format!("  {}", indent)));
                }
                base
            },
            None => {
                match self {
                    &Ast::Def(ref d) => {
                        let mut base = format!("{}{}", indent, ast_name!(self));
                        match d.name.get() {
                            Some(name) => {
                                base = format!("{}\n{}", base, name.to_string_tree_helper(format!("  {}", indent)));
                            }
                            None => {}
                        }
                        match d.expr.get() {
                            Some(expr) => {
                                base = format!("{}\n{}", base, expr.to_string_tree_helper(format!("  {}", indent)));
                            }
                            None => {}
                        }
                        base
                    }
                    &Ast::Let(ref l) => {
                        let mut base = format!("{}{}({})", indent, ast_name!(self), l.scope);
                        base = format!("{}\n{}  *Bindings", base, indent);
                        for value in l.bindings.borrow().iter() {
                            base = format!("{}\n{}{}", base, indent, value.0.to_string_tree_helper(format!("  {}", indent)));
                            base = format!("{}\n{}", base, value.1.to_string_tree_helper(format!("    {}", indent)));
                        }

                        base = format!("{}\n{}  *Body", base, indent);
                        for args in l.body.borrow().iter() {
                            base = format!("{}\n{}", base, args.to_string_tree_helper(format!("    {}", indent)));
                        }
                        base
                    }
                    &Ast::Lambda(ref l) => {
                        let mut base = format!("{}{}({})", indent, ast_name!(self), l.scope);
                        base = format!("{}\n{}  *Parameters", base, indent);
                        for args in l.arguments.borrow().iter() {
                            base = format!("{}\n{}", base, args.to_string_tree_helper(format!("    {}", indent)));
                        }
                        for b in l.body.borrow().iter() {
                            base = format!("{}\n{}", base, b.to_string_tree_helper(format!("  {}", indent)));
                        }
                        base
                    },
                    &Ast::DefMacro(ref l) => {
                        let mut base = format!("{}{}({})", indent, ast_name!(self), l.scope);
                        base = format!("{}\n{}", base, l.name.to_string_tree_helper(format!("  {}", indent)));
                        base = format!("{}\n{}  *Parameters", base, indent);
                        for args in l.arguments.borrow().iter() {
                            base = format!("{}\n{}", base, args.to_string_tree_helper(format!("    {}", indent)));
                        }
                        for b in l.body.borrow().iter() {
                            base = format!("{}\n{}", base, b.to_string_tree_helper(format!("  {}", indent)));
                        }
                        base
                    },
                    &Ast::Quote(ref q) => {
                        let mut base = format!("{}{}", indent, ast_name!(self));
                        match q.expr.get() {
                            Some(expr) => {
                                base = format!("{}\n{}", base, expr.to_string_tree_helper(format!("  {}", indent)));
                            }
                            None => {}
                        }
                        base
                    }
                    &Ast::If(ref i) => {
                        let mut base = format!("{}{}", indent, ast_name!(self));

                        match i.cond.get() {
                            Some(a) => {
                                base = format!("{}\n{}", base, a.to_string_tree_helper(format!("  {}", indent)));
                            },
                            None => {}
                        }

                        match i.then_body.get() {
                            Some(a) => {
                                base = format!("{}\n{}", base, a.to_string_tree_helper(format!("  {}", indent)));
                            },
                            None => {}
                        }

                        match i.else_body.get() {
                            Some(a) => {
                                base = format!("{}\n{}", base, a.to_string_tree_helper(format!("  {}", indent)));
                            },
                            None => {}
                        }
                        base
                    }
                    &Ast::Symbol(ref sym) => {
                        let mode = match sym.mode.get() {
                            SymbolMode::Var(depth) => {
                                match depth {
                                    SymbolDepth::Origin => {
                                        "Var(origin)".to_string()
                                    },
                                    SymbolDepth::Depth(d) => {
                                        format!("Var(depth = {})", d)
                                    }
                                }
                            }
                            SymbolMode::Parameter{index, depth} => {
                                match depth {
                                    SymbolDepth::Origin => {
                                        format!("Parameter(index = {}, origin)", index)
                                    },
                                    SymbolDepth::Depth(d) => {
                                        format!("Parameter(index = {}, depth = {})", index, d)
                                    }
                                }
                            }
                            SymbolMode::Unresolved => "Unresolved".to_string(),
                        };
                        format!("{}Symbol[mode = {}]({}, {})", indent, mode, sym.token(), sym.value)
                    }
                    &Ast::Integer(ref integer) => format!("{}Integer({}, {})", indent, integer.token(), integer.value),
                    &Ast::Double(ref d) => format!("{}Double({}, {})", indent, d.token(), d.value),
                    &Ast::String(ref s) => format!("{}String({}, '{}')", indent, s.token(), s.value),
                    &Ast::Keyword(ref k) => format!("{}Keyword({}, {})", indent, k.token(), k.value),
                    &Ast::Boolean(ref b) => format!("{}Boolean({}, {})", indent, b.token(), b.value),
                    &Ast::RegExp(ref r) => format!("{}RegExp({}, {})", indent, r.token(), r.value),
                    &Ast::Nil(ref n) => format!("{}Nil({})", indent, n.token()),
                    _ => {"".to_string()}
                }
            }
        }
    }
}

