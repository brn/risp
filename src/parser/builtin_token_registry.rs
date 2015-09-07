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
///! Builtin token definition
///!
///! Author Taketoshi Aono(brn) dobaw20@gmail.com


use parser::literal_buffer::{LiteralBuffer};


pub struct BuiltinTokenRegistry {
    defn: i64,
    def: i64,
    defmacro: i64
}


impl BuiltinTokenRegistry {
    pub fn new(literal_buffer: &LiteralBuffer) -> BuiltinTokenRegistry {
        let defn = literal_buffer.get("defn");
        let def  = literal_buffer.get("def");
        let defmacro = literal_buffer.get("defmacro");
        BuiltinTokenRegistry {
            defn: defn,
            def: def,
            defmacro: defmacro
        }
    }

    pub fn is_defn(&self, v: i64) -> bool {
        self.defn == v
    }

    pub fn is_def(&self, v: i64) -> bool {
        self.defn == v
    }

    pub fn is_defmacro(&self, v: i64) -> bool {
        self.defn == v
    }
}
