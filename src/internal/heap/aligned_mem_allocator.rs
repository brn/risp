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


use libc::{c_void, uintptr_t};
use std::{mem, ptr};
use internal::heap::virtual_mem_allocator::{VirtualHeapAllocator, prot, types, flags};
use internal::utils::Byte;

const ALLOCATE_ALIGNMENT: u64 = 8;
const MAX_RETRY: i32 = 10;

pub struct AlignedHeapAllocator;

impl AlignedHeapAllocator {
    /// Allocate aligned memory block.
    /// In this function, we allocate the bigger memory block than specified and roundup head block.
    /// To allocate the aligned memory block, we try to reserve bigger memory block
    /// and remove this block after get address of the target memory blocks,
    /// after we can get target memory block address, roundup this address and recommit this block,
    /// but, this memory block is already removed and other process may be already used,
    /// so we retry reserve another memory block until kMaxRetry count.
    pub fn allocate(block_size: u64, mut alignment: u64) -> *mut Byte {
        unsafe {
            if alignment <= ALLOCATE_ALIGNMENT {
                let addr = VirtualHeapAllocator::map(ptr::null_mut(), block_size,
                                                     prot::READ | prot::WRITE,
                                                     flags::ANONYMOUS | flags::PRIVATE,
                                                     types::RESERVE);
                // If ALLOCATED_ALIGNMENT is enough.
                return match addr {
                    Ok(p) => p as *mut Byte,
                    Err(e) => {panic!("{}", e);}
                };
            }

            // If ALLOCATE_ALIGNMENT is not enough alignment.
            let mut allocated_address: *mut Byte = ptr::null_mut();
            let mut aligned_address: *mut Byte = ptr::null_mut();
            let mut allocated_size: u64 = 0;
            let mut ret: *mut Byte = ptr::null_mut();
            let mut roundup: u64 = 0;
            let mut unused: u64 = 0;
            let mut retry: i32 = 0;

            // Reserve extra memory to roundup the head address.
            allocated_size = block_size + (alignment - ALLOCATE_ALIGNMENT);
            
            loop {
                retry += 1;

                if retry > MAX_RETRY {
                    panic!("Memory allocation failed.");
                }

                let allocation_result = VirtualHeapAllocator::map(
                    ptr::null_mut(), allocated_size,
                    prot::NONE,
                    flags::ANONYMOUS | flags::PRIVATE,
                    types::RESERVE);
                
                // First allocation, we reserve memory block that contains target range.
                // [--roundup---|----------target-----------]
                allocated_address = match allocation_result {
                    Ok(ptr) => ptr,
                    Err(e) => continue
                } as *mut u8;
                

                // Roundup allocated_address with the multiples of alignment.
                alignment -= 1;
                aligned_address = ((allocated_address.offset(alignment as isize) as uintptr_t) & !alignment) as *mut u8;

                // The roundup value of head addr.
                roundup = aligned_address.offset(-(allocated_address as isize)) as uintptr_t;

                // Restore alignment.
                alignment += 1;
                
                // The 'unused' is tail unused memory block.
                unused = alignment - ALLOCATE_ALIGNMENT - roundup;
                // Remove reserved block.
                if !AlignedHeapAllocator::remap(aligned_address, allocated_address, allocated_size, block_size, unused, roundup) {
                    continue;
                }
                
                return aligned_address;
            }
        }
    }


    #[cfg(unix)]
    fn remap(aligned_address: *mut Byte, allocated_address: *mut Byte, allocated_size: u64, block_size: u64, unused: u64, roundup: u64) -> bool {
        use libc::{mprotect};
        use libc::{PROT_READ, PROT_WRITE};
        unsafe {
            if roundup > 0 {
                VirtualHeapAllocator::unmap(allocated_address as *mut c_void, roundup);
            }
            
            if unused > 0 {
                VirtualHeapAllocator::unmap(allocated_address.offset((allocated_size - unused) as isize) as *mut c_void, unused);
            }
            mprotect(aligned_address as *mut c_void, block_size, PROT_READ | PROT_WRITE);
        }
        true
    }


    #[cfg(windows)]
    fn remap(allocated_address: *mut u8, allocated_size: u64, unused: u64, roundup: u64) -> bool {
        unsafe {
            VirtualHeapAllocator::unmap(allocated_address, roundup, types::DECOMMIT);
            VirtualHeapAllocator::unmap(allocated_address.offset((allocated_size - unused) as isize), unused, types::DECOMMIT);
            // Commit found memory block again.
            // But,this commit may be failed,
            // because other process may use this block.
            let tmp = VirtualHeapAllocator::map(aligned_address, block_size,
                                                prot::READ | prot::WRITE,
                                                flags::ANONYMOUS | flags::PRIVATE,
                                                types::RESERVE);
            let ret = match tmp {
                Ok(ref mut ptr) => ptr,
                Err(e) => {panic!(e.message());}
            };

            // Check commited memory block address is less than the alignedAddress.
            if ret != aligned_address {
                VirtualHeapAllocator::unmap(allocated_address, block_size + roundup + unused);
                return false;
            }
            true
        }
    }
}


#[cfg(test)]
mod test {
    use libc::{uintptr_t, c_void};
    use super::*;
    use std::ptr;
    use internal::utils::Byte;
 
    struct Test {
        x: u32,
        x2: u32
    }
    impl Test {
        pub fn new(addr: *mut Byte) -> *mut Test {
            unsafe {
                assert!(!addr.is_null());
                let mem = addr as *mut Test;
                ptr::write(mem, Test {x:10, x2: 20});
                mem
            }
        }
        pub fn x(&self) -> u32 {self.x}
        pub fn x2(&self) -> u32 {self.x2}
    }
    #[test]
    fn test() {
        unsafe {
            let addr = AlignedHeapAllocator::allocate(kb!(32), kb!(64));
            let t = Test::new(addr);
            assert!((*t).x() == 10);
            assert!(addr as uintptr_t == (addr.offset(kb!(63)) as uintptr_t) & !0xFFFF);
        }
    }
}
