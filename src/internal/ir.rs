///! IR definitions.
///!
///! Taketoshi Aono

// use std;
// use std::rc::Rc;
// use std::cell::{Cell, RefCell, Ref, RefMut};
// use parser::token::Token;
// use parser::sourceinfo::SourceInfo;
// use std::string::ToString;
// use std::fmt::Display;

// pub type IRPtr<'a> = Box<IR<'a>>;
// pub type ChildrenMut<'a> = RefMut<'a, Vec<IRPtr<'a>>>;
// pub type Children<'a> = Ref<'a, Vec<IRPtr<'a>>>;
// pub type ChildrenRef<'a> = RefCell<Vec<IRPtr<'a>>>;
// pub type Child<'a> = Cell<Option<IRPtr<'a>>>;
// pub type Parent<'a> = Cell<Option<&'a IR<'a>>>;

pub struct IR<'a> {
    x: &'a i32
}

// trait BasicIR<'a> {
//     fn set_parent(&self, new_parent: &'a IR<'a>) {
//         self.parent.set(new_parent);
//     }


//     fn parent(&self) -> Option<&'a IR<'a>> {
//         self.parent.get()
//     }
// }


// trait BasicForm<'a> {
//     fn add_child(&self, new_child: &'a IR<'a>) {
//         match self.first_child {
//             Some(child) => {
//                 self.last_child.unwrap().set_next_sibling
//             },
//             None => self.first_child.set(Some(new_child))
//         }
//     }

    
//     fn remove_child(&self, child: &'a IR<'a>) {
//         assert!(child.parent() == self);
//         child.set_prev_sibling(Option::None);
//         match child.prev_sibling() {
//             None => {}
//             Some(p) => p.set_next_sibling(Option::None)
//         }
//         child.set_parent(Option::None);
//     }


//     fn first_child(&self) -> &'a IR<'a>{
//         self.first_child
//     }


//     fn last_child(&self) -> &'a IR<'a> {
//         self.last_child
//     }
// }


// struct IR<'a> {
//     parent: Parent<'a>,
//     token: Token<'a>
// }

// impl<'a> BasicIR<'a> for IR<'a> {}


// struct Form<'a> {
//     parent: Parent<'a>,
//     token: Token<'a>,
//     first_child: Child<'a>,
//     last_child: Child<'a>,
//     next_sibling: Child<'a>,
//     prev_sibling: Child<'a>
// }

// impl<'a> BasicIR<'a> for Form<'a> {}
// impl<'a> BasicForm<'a> for Form<'a> {}


// /// Get children from Vec if current self is form like ir.
// macro_rules! children {
//     ($this:expr) => {
//         match $this {
//             IR::Root         {token, ref first_child, ref parent} |
//             IR::Module       {token, ref first_child, ref parent} |
//             IR::Form         {token, ref first_child, ref parent} |
//             IR::List         {token, ref first_child, ref parent} |
//             IR::Vector       {token, ref first_child, ref parent} |
//             IR::Map          {token, ref first_child, ref parent} |
//             IR::Set          {token, ref first_child, ref parent} |
//             IR::Lambda       {token, ref first_child, ref parent} => (token, Option::Some(first_child), parent),
//             IR::Keyword      {token, ref parent} |
//             IR::MacroKeyword {token, ref parent} |
//             IR::Literal      {token, ref parent} |
//             IR::Symbol       {token, ref parent} => (token, Option::None, parent)
//         }
//     }
// }


// pub enum IR<'a> {
//     Root {
//         parent: Parent<'a>,
//         token: Token<'a>,
//         first_child: Child<'a>,
//         last_child: Child<'a>,
//     },
//     Module {
//         parent: Parent<'a>,
//         token: Token<'a>,
//         first_child: Child<'a>
//     },
//     Form {
//         parent: Parent<'a>,
//         token: Token<'a>,
//         first_child: Child<'a>
//     },
//     List {
//         parent: Parent<'a>,
//         token: Token<'a>,
//         first_child: Child<'a>
//     },
//     Vector {
//         parent: Parent<'a>,
//         token: Token<'a>,
//         first_child: Child<'a>
//     },
//     Map {
//         parent: Parent<'a>,
//         token: Token<'a>,
//         first_child: Child<'a>
//     },
//     Set {
//         parent: Parent<'a>,
//         token: Token<'a>,
//         first_child: Child<'a>
//     },
//     Lambda {
//         parent: Parent<'a>,
//         token: Token<'a>,
//         first_child: Child<'a>
//     },
//     Keyword {
//         parent: Parent<'a>,
//         token: Token<'a>
//     },
//     MacroKeyword {
//         parent: Parent<'a>,
//         token: Token<'a>
//     },
//     Literal {
//         parent: Parent<'a>,
//         token: Token<'a>
//     },
//     Symbol {
//         parent: Parent<'a>,
//         token: Token<'a>
//     }
// }


// impl<'a> std::fmt::Display for IR<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{}", self.to_string_tree("".to_string()))
//     }
// }


// impl<'a> IR<'a> {
//     pub fn new_root(token: Token<'a>) -> IR<'a> {
//         IR::Root {token: token, first_child: Cell::new(Option::None), parent: Cell::new(Option::None)}
//     }


//     pub fn new_module(token: Token<'a>) -> IR<'a> {
//         IR::Module {token: token, first_child: Cell::new(Option::None), parent: Cell::new(Option::None)}
//     }


//     pub fn new_form(token: Token<'a>) -> IR<'a> {
//         IR::Form {token: token, first_child: Cell::new(Option::None), parent: Cell::new(Option::None)}
//     }


//     pub fn new_list(token: Token<'a>) -> IR<'a> {
//         IR::List {token: token, first_child: Cell::new(Option::None), parent: Cell::new(Option::None)}
//     }

    
//     pub fn new_vector(token: Token<'a>) -> IR<'a> {
//         IR::Vector {token: token, first_child: Cell::new(Option::None), parent: Cell::new(Option::None)}
//     }


//     pub fn new_map(token: Token<'a>) -> IR<'a> {
//         IR::Map {token: token, first_child: Cell::new(Option::None), parent: Cell::new(Option::None)}
//     }

    
//     pub fn new_set(token: Token<'a>) -> IR<'a> {
//         IR::Set {token: token, first_child: Cell::new(Option::None), parent: Cell::new(Option::None)}
//     }


//     pub fn new_lambda(token: Token<'a>) -> IR<'a> {
//         IR::Lambda {token: token, first_child: Cell::new(Option::None), parent: Cell::new(Option::None)}
//     }


//     pub fn new_keyword(token: Token<'a>) -> IR<'a> {
//         IR::Keyword {token: token, parent: Cell::new(Option::None)}
//     }


//     pub fn new_macro_keyword(token: Token<'a>) -> IR<'a> {
//         IR::MacroKeyword {token: token, parent: Cell::new(Option::None)}
//     }


//     pub fn new_literal(token: Token<'a>) -> IR<'a> {
//         IR::Literal {token: token, parent: Cell::new(Option::None)}
//     }


//     pub fn new_symbol(token: Token<'a>) -> IR<'a> {
//         IR::Symbol {token: token, parent: Cell::new(Option::None)}
//     }
    

//     pub fn add_child(&self, ir: IRPtr<'a>) {
//         match children!(*self) {
//             (t, Some(child), parent) => {
//                 children.borrow_mut().push(ir);
//                 children.borrow().last().unwrap().set_parent(self);
//             },
//             (t, None, ref parent) => {panic!("Specified ir cannot have children.");}
//         }
//     }


//     pub fn children(&'a self) -> Option<Children<'a>> {
//         match children!(*self) {
//             (t, Some(ref children), parent) => Some(children.borrow()),
//             (t, None, ref parent) => None
//         }
//     }


//     pub fn set_parent(&self, new_parent: &'a IR<'a>) {
//         match children!(*self) {
//             (t, Some(ref children), parent) => {
//                 parent.set(Option::Some(new_parent));
//             }
//             (t, None, parent) => {
//                 parent.set(Option::Some(new_parent));
//             }
//         }
//     }


//     pub fn parent(&self) -> Option<&'a IR<'a>> {
//         match children!(*self) {
//             (t, Some(ref children), parent) => parent.get(),
//             (t, None, parent) => parent.get()
//         }
//     }


//     fn to_string_tree(&self, indent: String) -> String {
//         match children!(*self) {
//             (t, Some(ref children), _) => {
//                 let mut base = format!("{}{}", indent, self.name());
//                 for child in children.borrow().iter() {
//                     base = format!("{}\n{}", base, child.to_string_tree(format!("  {}", indent)));
//                 }
//                 base
//             },
//             (t, None, _) => {
//                 format!("{}{}({})", indent, self.name(), t)
//             }
//         }
//     }


//     fn is_form(&self) -> bool {
//         match children!(*self) {
//             (_, Some(_), _) => true,
//             (_, None, _) => false
//         }
//     }
    
    
//     fn name(&self) -> &'static str {
//         match *self {
//             IR::Root         {ref token, ref children, ref parent} => "Root",
//             IR::Module       {ref token, ref children, ref parent} => "Module",
//             IR::Form         {ref token, ref children, ref parent} => "Form",
//             IR::List         {ref token, ref children, ref parent} => "List",
//             IR::Vector       {ref token, ref children, ref parent} => "Vector",
//             IR::Map          {ref token, ref children, ref parent} => "Map",
//             IR::Set          {ref token, ref children, ref parent} => "Set",
//             IR::Lambda       {ref token, ref children, ref parent} => "Lambda",
//             IR::Keyword      {ref token, ref parent} => "Keyword",
//             IR::MacroKeyword {ref token, ref parent} => "MacroKeyword",
//             IR::Literal      {ref token, ref parent} => "Literal",
//             IR::Symbol       {ref token, ref parent} => "Symbol"
//         }
//     }
// }
