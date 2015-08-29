///!
///!
///!

use std;
use std::result::Result;
use nix::errno;
use nix::sys::mman;
use libc::c_void;
use libc;
use libc::strerror;

#[cfg(windows)] use libc::consts::os::extra::{PAGE_EXECUTE, PAGE_EXECUTE_READWRITE, PAGE_READWRITE, PAGE_READONLY};
#[cfg(windows)] use libc::types::os::arch::extra;


pub mod prot {
    pub const NONE: i32 = 0;
    pub const READ: i32 = 0x1;
    pub const WRITE: i32 = 0x2;
    pub const EXEC: i32 = 0x4;
}


pub mod flags {
    pub const NONE: i32 = 0;
    pub const ANONYMOUS: i32 = 0x1;
    pub const SHARED: i32 = 0x2;
    pub const PRIVATE: i32 = 0x4;
    pub const FIXED: i32 = 0x8;
}


pub mod types {
    pub const COMMIT: i32 = 0x1;
    pub const DECOMMIT: i32 = 0x2;
    pub const RESERVE: i32 = 0x4;
    pub const RELEASE: i32 = 0x8;
}


pub struct AllocationError {
    message: &'static str
}

impl<'a> std::fmt::Display for AllocationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}


impl AllocationError {
    pub fn new(message: &'static str) -> AllocationError {
        AllocationError {
            message: message
        }
    }


    pub fn message(&self) -> &str {
        &self.message
    }
}


pub struct VirtualHeapAllocator;
impl VirtualHeapAllocator {
    #[cfg(unix)]
    #[inline]
    pub fn map(addr: *mut c_void, size: u64, prot: i32, flags: i32, t: i32) -> Result<*mut c_void, AllocationError> {
        unsafe {
            return VirtualHeapAllocator::map_posix(addr, size, prot, flags);
        }
    }


    #[cfg(windows)]
    #[inline]
    pub fn map(addr: *mut c_void, size: u64, prot: i32, flags: i32, t: i32) -> Result<*mut c_void, AllocationError> {
        unsafe {
            return VirtualHeapAllocator::map_win32(addr, size, prot, flags, t);
        }
    }

    
    #[cfg(unix)]
    #[inline]
    pub fn unmap(addr: *mut c_void, size: u64) {
        unsafe {
            VirtualHeapAllocator::unmap_posix(addr, size);
        }
    }


    #[cfg(windows)]
    #[inline]
    pub fn unmap(addr: *mut c_void, size: u64) {
        unsafe {
            VirtualHeapAllocator::unmap_win32(addr, size);
        }
    }


    #[cfg(unix)]
    unsafe fn map_posix(addr: *mut c_void, size: u64, prot: i32, flags: i32) -> Result<*mut c_void, AllocationError> {
        let mut mmap_prot: i32 = 0;
        let mut mmap_flags: i32 = 0;

        if prot != prot::NONE {
            if (prot & prot::READ) == prot::READ {
                mmap_prot |= mman::PROT_READ;
            }

            if (prot & prot::WRITE) == prot::WRITE {
                mmap_prot |= mman::PROT_WRITE;
            }

            if (prot & prot::EXEC) == prot::EXEC {
                mmap_prot |= mman::PROT_EXEC;
            }
        } else {
            mmap_prot = mman::PROT_NONE;
        }

        
        if flags != flags::NONE {
            if (flags & flags::ANONYMOUS) == flags::ANONYMOUS {
                mmap_flags |= libc::MAP_ANON;
            }

            if (flags & flags::SHARED) == flags::SHARED {
                mmap_flags |= mman::MAP_SHARED;
            }

            if (flags & flags::PRIVATE) == flags::PRIVATE {
                mmap_flags |= mman::MAP_PRIVATE;
            }
            
            if addr.is_null() && (flags & flags::FIXED) == flags::FIXED {
                mmap_flags |= mman::MAP_FIXED;
            }
        }

        match mman::mmap(addr, size, mmap_prot, mmap_flags, -1, 0) {
            Ok(h) => Result::Ok(h),
            Err(_) => Result::Err(AllocationError::new(VirtualHeapAllocator::get_last_error_posix()))
        }
    }


    #[cfg(windows)]
    unsafe fn map_win32(addr: *mut c_void, size: u64, prot: i32, flags: i32, t: i32) -> Result<*mut c_void, AllocationError> {
        let fl_protect: extra::DWORD = 0;
        let fl_allocation_type: extra::DWORD = 0;

        if (prot != prot::NONE) {
            if ((prot & 0x7) == 0x7 || (prot & 0x6) == 0x6) {
                fl_protect = PAGE_EXECUTE_READWRITE;
            } else if ((prot & 0x5) == 0x5) {
                fl_protect = PAGE_EXECUTE_READ;
            } else if ((prot & prot::WRITE) == prot::WRITE ||
                       (prot & 0x3) == 0x3) {
                fl_protect = PAGE_READWRITE;
            } else if ((prot & prot::READ) == prot::READ) {
                fl_protect = PAGE_READONLY;
            } else if ((prot & prot::EXEC) == prot::EXEC) {
                fl_protect = PAGE_EXECUTE;
            }
        } else {
            fl_protect = PAGE_READONLY;
        }
        
        if (t == types::COMMIT || flags != flags::NONE) {
            fl_allocation_type = MEM_COMMIT;
        } else if (t == types::RESERVE) {
            fl_allocation_type = MEM_RESERVE;
        }

        let ret: &mut c_void = libc::VirtualAlloc(addr, size, fl_allocation_type, fl_protect);
        
        if (ret == std::ptr::null()) {
            return Result::Err(VirtualHeapAllocator::get_last_error_win());
        }
        Result::Ok(ret)
    }


    #[cfg(unix)]
    unsafe fn unmap_posix(addr: *mut c_void, size: u64) {
        match mman::munmap(addr, size) {
            Ok(t) => {}
            Err(err) => {panic!(err.errno().desc());}
        }
    }


    #[cfg(windows)]
    unsafe fn unmap_posix(addr: *mut c_void, size: u64) {
        libc::VirtualFree(addr, size);
    }


    #[cfg(unix)]
    fn get_last_error_posix() -> &'static str {
        errno::from_i32(errno::errno()).desc()
    }


    #[cfg(windows)]
    fn get_last_error_win() -> String {
        let msg_buffer: libc::LPVOID;
        FormatMessage(
            libc::FORMAT_MESSAGE_ALLOCATE_BUFFER | 
            libc::FORMAT_MESSAGE_FROM_SYSTEM | 
            libc::FORMAT_MESSAGE_IGNORE_INSERTS,
            std::ptr::null(), libc::GetLastError(),
            libc::MAKELANGID(libc::LANG_NEUTRAL, libc::SUBLANG_DEFAULT), 
            &msg_buffer as libc::LPTSTR, 0, std::ptr::null());
        let ret = String(msg_buffer as &str);
        libc::LocalFree(msg_buffer);
        ret
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn mmap_test() {
        use std;
        use super::*;
        use libc::c_void;
        unsafe {
            match VirtualHeapAllocator::map(std::ptr::null_mut(), 1024, prot::READ | prot::WRITE | prot::EXEC, flags::ANONYMOUS | flags::PRIVATE, types::COMMIT) {
                Ok(ptr) => {},
                Err(e) => {panic!("{}", e)}
            }
        }
    }
}
