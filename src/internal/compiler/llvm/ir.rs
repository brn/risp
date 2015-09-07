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

use llvm::prelude::*;
use llvm::core::*;
use llvm::target_machine::*;
use llvm::target::*;
use llvm::analysis::*;
use llvm::transforms::ipo::*;
use llvm::transforms::scalar::*;
use llvm::transforms::vectorize::*;
use llvm::execution_engine::*;
use std::ffi::{CString};

use internal::compiler::llvm::context::{IRContext};

pub struct IR<'a> {
    builder: LLVMBuilderRef,
    context: &'a IRContext
}


impl<'a> IR<'a> {
    pub fn new(context: &'a IRContext) -> IR<'a> {
        unsafe {
            IR {
                builder: LLVMCreateBuilderInContext(context.context()),
                context: context
            }
        }
    }


    pub fn builder(&self) -> LLVMBuilderRef {
        self.builder
    }


    pub fn doublety() -> LLVMTypeRef {
        unsafe {
            LLVMDoubleType()
        }
    }
    

    pub fn int32ty() -> LLVMTypeRef {
        unsafe {
            LLVMInt32Type()
        }
    }
    

    pub fn function_type(ty: LLVMTypeRef, args: &mut LLVMTypeRef, param_count: u32, var_args: bool) -> LLVMTypeRef {
        unsafe {
            LLVMFunctionType(ty, args as *mut LLVMTypeRef, param_count, if var_args {1} else {0})
        }
    }
    

    pub fn add_function(&self, name: CString, function_type: LLVMTypeRef) -> LLVMValueRef {
        unsafe {
            LLVMAddFunction(self.context.module(), name.as_ptr(), function_type)
        }
    }


    pub fn append_basic_block(&self, name: CString, function: LLVMValueRef) -> LLVMBasicBlockRef {
        unsafe {
            LLVMAppendBasicBlockInContext(self.context.context(), function, name.as_ptr())
        }
    }


    pub fn position_at_end(&self, block: LLVMBasicBlockRef) {
        unsafe {
            LLVMPositionBuilderAtEnd(self.builder, block);
        }
    }


    pub fn build_add(&self, name: CString, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        unsafe {
            LLVMBuildAdd(self.builder, lhs, rhs, name.as_ptr())
        }
    }


    pub fn build_fadd(&self, name: CString, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        unsafe {
            LLVMBuildFAdd(self.builder, lhs, rhs, name.as_ptr())
        }
    }


    pub fn ret(&self, v: LLVMValueRef) {
        unsafe {
            LLVMBuildRet(self.builder, v);
        }
    }


    pub fn get_param(v: LLVMValueRef, index: u32) -> LLVMValueRef {
        unsafe {
            LLVMGetParam(v, index)
        }
    }
}
