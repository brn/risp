///! Literal value buffer.
///!
///! Author Taketoshi Aono

use std::vec::Vec;
use std::collections::{HashMap};
use std::cell::{RefCell, Cell, Ref};
use std::hash::{Hash, Hasher};
use std::cmp::{Eq, PartialEq};
use std::borrow::{Borrow};
use internal::heap::zone::{ZoneAllocator, ZoneObject};

pub struct LiteralBuf {
    literal: String
}


impl ZoneObject<LiteralBuf> for LiteralBuf {}


impl<'a> LiteralBuf {
    pub fn new(za: &'a ZoneAllocator, literal: &str) -> &'a LiteralBuf {
        za.alloc(LiteralBuf {
            literal: literal.to_string()
        })
    }

    pub fn literal(&self) -> &str {
        &self.literal
    }
}


impl Hash for LiteralBuf {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.literal.hash(state);
    }
}


impl PartialEq for LiteralBuf {
    fn eq(&self, other: &LiteralBuf) -> bool {
        other.literal() == self.literal()
    }

    
    fn ne(&self, other: &LiteralBuf) -> bool {
        !self.eq(other)
    }
}


impl Eq for LiteralBuf {}


impl<'a> Borrow<str> for &'a LiteralBuf {
    fn borrow(&self) -> &str {
        self.literal()
    }
}


pub struct LiteralBuffer<'a> {
    map: RefCell<HashMap<&'a LiteralBuf, i64>>,
    literal_list: RefCell<Vec<&'a LiteralBuf>>,
    zone_allocator: &'a ZoneAllocator
}

impl<'a> LiteralBuffer<'a> {
    pub fn new(zone_allocator: &'a ZoneAllocator) -> LiteralBuffer<'a> {
        LiteralBuffer {
            map: RefCell::new(HashMap::new()),
            literal_list: RefCell::new(Vec::new()),
            zone_allocator: zone_allocator
        }
    }
    
    pub fn get(&self, key: &str) -> i64 {
        let has = self.map.borrow().contains_key(key);
        if has {
            return match self.map.borrow().get(key) {
                Some(id) => *id,
                None => self.insert_new_id(key)
            };
        }
        self.insert_new_id(key)
    }


    pub fn find(&self, key: i64) -> &'a str {
        if (key as usize) < self.literal_list.borrow().len() {
            return self.literal_list.borrow()[key as usize].literal();
        }
        ""
    }
    

    fn insert_new_id(&self, key: &str) -> i64 {
        let literal = LiteralBuf::new(self.zone_allocator, key);
        let id = self.literal_list.borrow().len() as i64;
        self.map.borrow_mut().insert(literal, id);
        self.literal_list.borrow_mut().push(literal);
        id
    }
}
