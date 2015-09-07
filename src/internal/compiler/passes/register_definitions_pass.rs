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
///! Traversable trait definition
///!
///! Author Taketoshi Aono(brn) dobaw20@gmail.com

use internal::ast::*;
use internal::heap::gc::gc::*;
use parser::builtin_token_registry::{BuiltinTokenRegistry};
use internal::compiler::llvm::context::{IRContext};
use internal::compiler::llvm::ir::{IR};
use parser::literal_buffer::{LiteralBuffer};


struct RegisterDefinitionPass<'a, 'b> {
    builtin_token_registry: &'b BuiltinTokenRegistry,
    ir: &'b IR<'b>,
    gc: &'a GC
}


impl<'a, 'b> AstVisitor<'a, bool> for RegisterDefinitionPass<'a, 'b> {
    fn visit_module(&self, ast: &'a Ast<'a>) -> bool {true}

    fn visit_map(&self, ast: &'a Ast<'a>) -> bool {true}

    fn visit_tag(&self, ast: &'a Ast<'a>) -> bool {true}

    fn visit_set(&self, ast: &'a Ast<'a>) -> bool {true}

    fn visit_list(&self, ast: &'a Ast<'a>) -> bool {true}

    fn visit_lambda_sugar(&self, ast: &'a Ast<'a>) -> bool {true}

    fn visit_quote(&self, ast: &'a Ast<'a>) -> bool {true}

    fn visit_if(&self, ast: &'a Ast<'a>) -> bool {true}

    fn visit_def(&self, ast: &'a Ast<'a>) -> bool {true}

    fn visit_vector(&self, ast: &'a Ast<'a>) -> bool {true}

    fn visit_module_ref(&self, ast: &'a Ast<'a>) -> bool {true}

    fn visit_lambda(&self, ast: &'a Ast<'a>) -> bool {true}

    fn visit_defmacro(&self, ast: &'a Ast<'a>) -> bool {true}

    fn visit_integer(&self, ast: &'a Ast<'a>) -> bool {true}

    fn visit_double(&self, ast: &'a Ast<'a>) -> bool {true}

    fn visit_string(&self, ast: &'a Ast<'a>) -> bool {true}

    fn visit_symbol(&self, ast: &'a Ast<'a>) -> bool {true}

    fn visit_keyword(&self, ast: &'a Ast<'a>) -> bool {true}

    fn visit_boolean(&self, ast: &'a Ast<'a>) -> bool {true}

    fn visit_regexp(&self, ast: &'a Ast<'a>) -> bool {true}

    fn visit_lambda_param(&self, ast: &'a Ast<'a>) -> bool {true}

    fn visit_uchar(&self, ast: &'a Ast<'a>) -> bool {true}

    fn visit_nil(&self, ast: &'a Ast<'a>) -> bool {true}
}


impl<'a, 'b> RegisterDefinitionPass<'a, 'b> {
    pub fn new(gc: &'a GC, ir: &'b IR<'b>,
               builtin_token_registry: &'b BuiltinTokenRegistry) -> RegisterDefinitionPass<'a, 'b> {
        RegisterDefinitionPass {
            builtin_token_registry: builtin_token_registry,
            ir: ir,
            gc: gc
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use internal::compiler::passes::register_definitions_pass::{RegisterDefinitionPass};
    use internal::compiler::llvm::context::{IRContext};
    use internal::compiler::llvm::ir::{IR};
    use internal::ast::*;
    use internal::runtime::math::{Math};
    use parser::literal_buffer::{LiteralBuffer};
    use parser::builtin_token_registry::{BuiltinTokenRegistry};
    use std::ffi::CString;

    #[test]
    fn test() {
        use parser::token;
        use parser::moduleinfo;
        use parser::parseerror;
        use parser::parser;
        use internal::heap::zone::{ZoneAllocator};
        use internal::heap::gc::gc::{GC};
        IRContext::initialize();
        let gc = GC::new();
        let zone_allocator = ZoneAllocator::new();
        let module_info = moduleinfo::ModuleInfo::new("test/test_files/test.rp");
        let lb = LiteralBuffer::new(&zone_allocator);
        let builtin_token_registry = BuiltinTokenRegistry::new(&lb);
        let c = IRContext::new(CString::new("test").unwrap());
        let ir = IR::new(&c);
        let m = Math::new(&ir);
        c.dump();
        {
            let pass = RegisterDefinitionPass::new(&gc, &ir, &builtin_token_registry);
            let parser = parser::Parser::new_from_file(&module_info, &lb, &zone_allocator);
            let ret = parser.parse();
            match ret {
                Ok(r) => {
                    r.visit(&pass);
                    return;
                },
                Err(e) => println!("{}", e)
            }
        }
    }
}
