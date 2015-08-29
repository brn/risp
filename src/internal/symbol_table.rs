///!
///!
///! Author Taketoshi Aono

use parser::literal_buffer::LiteralBuffer;
use std::cell::{RefCell};
use std::collections::HashMap;


pub struct SymbolTable<'a> {
    literal_buffer: LiteralBuffer,
    table: RefCell<HashMap<i32, IR<'a>>>
}


impl<'a> SymbolTable<'a> {
    pub fn new() -> SymbolTable<'a> {
        SymbolTable {
            literal_buffer: LiteralBuffer::new(),
            table: RefCell::new(HashMap::new())
        }
    }
}
