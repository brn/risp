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
///! llvm context utility definition
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

pub struct IRContext {
    context: LLVMContextRef,
    module: LLVMModuleRef
}


impl IRContext {
    pub fn new(name: CString) -> IRContext {
        unsafe {
            let context = LLVMGetGlobalContext();
            IRContext {
                context: context,
                module: LLVMModuleCreateWithNameInContext(name.as_ptr(), context)
            }
        }
    }

    pub fn initialize() {
        unsafe {
            LLVM_InitializeNativeTarget();
            LLVM_InitializeAllAsmPrinters();
            LLVMLinkInMCJIT();
        }
    }

    pub fn context(&self) -> LLVMContextRef {
        self.context
    }


    pub fn module(&self) -> LLVMModuleRef {
        self.module
    }


    pub fn dump(&self) {
        unsafe {
            LLVMDumpModule(self.module);
        }
    }
}
