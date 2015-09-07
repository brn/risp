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

use llvm::core::*;
use llvm::prelude::*;
use internal::compiler::llvm::ir::{IR};
use std::ffi::{CString};

pub struct Math<'a> {
    ir: &'a IR<'a>,
    add_int32: LLVMValueRef,
    add_double: LLVMValueRef
}


impl<'a> Math<'a> {
    pub fn new(ir: &'a IR<'a>) -> Math {
        Math {
            ir: ir,
            add_int32: Math::init_add_32(ir),
            add_double: Math::init_add_fp(ir),
        }
    }


    fn init_add_32(ir: &'a IR<'a>) -> LLVMValueRef {
        unsafe {
            let ty = IR::int32ty();
            let builder = ir.builder();
            let mut param_types = [ty, ty];
            let function_type = IR::function_type(ty, &mut param_types[0], 2, false);
            let function = ir.add_function(CString::new("add_int32").unwrap(), function_type);
            let bb = ir.append_basic_block(CString::new("entry").unwrap(), function);
            ir.position_at_end(bb);
            let tmp = ir.build_add(CString::new("result").unwrap(), IR::get_param(function, 0), IR::get_param(function, 1));
            ir.ret(tmp);
            function
        }
    }


    fn init_add_fp(ir: &'a IR<'a>) -> LLVMValueRef {
        unsafe {
            let ty = IR::doublety();
            let builder = ir.builder();
            let mut param_types = [ty, ty];
            let function_type = IR::function_type(ty, &mut param_types[0], 2, false);
            let function = ir.add_function(CString::new("add_fp").unwrap(), function_type);
            let bb = ir.append_basic_block(CString::new("entry").unwrap(), function);
            ir.position_at_end(bb);
            let tmp = ir.build_fadd(CString::new("result").unwrap(), IR::get_param(function, 0), IR::get_param(function, 1));
            ir.ret(tmp);
            function
        }
    }
}
