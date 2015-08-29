///! Module information.
///!
///! Author Taketoshi Aono

use std;
use std::fmt::{Display, Formatter};

pub struct ModuleInfo {
    filename: String
}


impl Display for ModuleInfo {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.filename)
    }
}


impl ModuleInfo {
    pub fn new(filename: &str) -> ModuleInfo {
        ModuleInfo {
            filename: filename.to_string()
        }
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }
}
