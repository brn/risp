///! Main
///!
///! Author Taketoshi Aono
///!

extern crate risp;

use risp::parser::token;
use risp::parser::literal_buffer;
use risp::parser::moduleinfo;
use risp::parser::parseerror;
use risp::parser::parser;
use risp::internal::heap::zone::{ZoneAllocator};

fn main() {
    let zone_allocator = ZoneAllocator::new();
    let module_info = moduleinfo::ModuleInfo::new("test/test_files/test.rp");
    let lb = literal_buffer::LiteralBuffer::new(&zone_allocator);
    let parser = parser::Parser::new_from_file(&module_info, &lb, &zone_allocator);
    {
        let ret = parser.parse();
        match ret {
            Ok(r) => {println!("{}", r.to_string_tree());}
            Err(e) => {println!("{}", e);}
        }
    }
}
