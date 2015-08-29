///! Source file information definiton.
///!
///! Author Taketoshi Aono

use parser::moduleinfo::ModuleInfo;
use std;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
pub struct SourceInfo<'a> {
    pos: i32,
    line: i32,
    module_info: &'a ModuleInfo
}


impl<'a> Display for SourceInfo<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}:{:?}:{:?}", self.module_info, self.pos, self.line)
    }
}


impl<'a> SourceInfo<'a> {
    pub fn new(pos: i32, line: i32, module_info: &'a ModuleInfo) -> SourceInfo<'a> {
        SourceInfo {
            pos: pos,
            line: line,
            module_info: module_info
        }
    }

    pub fn pos(self) -> i32 {
        self.pos
    }

    
    pub fn line(self) -> i32 {
        self.line
    }

    
    pub fn filename(self) -> &'a str {
        self.module_info.filename()
    }
}
