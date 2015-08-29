///!
///!
///!

use std::{mem,ptr};
use std::cell::{Cell};
use libc::{c_void};
use internal::heap::virtual_mem_allocator::{VirtualHeapAllocator, prot, flags, types};
use internal::utils::{Byte, PAGE_SIZE};

struct ZoneHeap {
    heap: Cell<*mut Byte>,
    size: usize,
    used: Cell<usize>,
    next: Cell<*const ZoneHeap>
}


#[inline]
fn zone_heap_size() -> usize {
    mem::size_of::<ZoneHeap>()
}


impl ZoneHeap {
    pub fn new(addr: *mut Byte, size: usize) -> *const ZoneHeap {
        unsafe {
            let zone = ZoneHeap {heap: Cell::new(addr.offset(zone_heap_size() as isize)), size: size, used: Cell::new(mem::size_of::<ZoneHeap>()), next: Cell::new(ptr::null_mut() as *mut ZoneHeap)};
            let zone_ptr = addr as *mut ZoneHeap;
            ptr::write(zone_ptr, zone);
            zone_ptr
        }
    }

    #[inline]
    pub fn has_enough_size(&self, size: usize) -> bool {
        self.size - self.used.get() > size
    }

    
    pub fn alloc(&self, size: usize) -> *mut Byte {
        unsafe {
            let addr = self.heap.get();
            self.used.set(self.used.get() + size);
            self.heap.set(self.heap.get().offset(size as isize));
            addr
        }
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn set_next(&self, next: *const ZoneHeap) {
        self.next.set(next);
    }

    #[inline]
    pub fn next(&self) -> *const ZoneHeap {
        self.next.get()
    }


    pub fn destroy(&self) {
        unsafe {
            let target = self.heap.get().offset(-self.used.get() as isize) as *mut c_void;
            VirtualHeapAllocator::unmap(target, self.size as u64);
        }
    }
}


pub trait ZoneObject<T> {}


pub struct ZoneAllocator {
    size: usize,
    zone: Cell<*const ZoneHeap>,
    head: Cell<*const ZoneHeap>,
    destructed: Cell<bool>
}


impl ZoneAllocator {
    pub fn new() -> ZoneAllocator {
        unsafe {
            let zone = ZoneAllocator::create_zone(PAGE_SIZE);
            ZoneAllocator {
                size: (*zone).size(),
                zone: Cell::new(zone),
                head: Cell::new(zone),
                destructed: Cell::new(false)
            }
        }
    }


    pub fn alloc<T: ZoneObject<T>>(&self, v: T) -> &T {
        unsafe {
            let size = mem::size_of::<T>();
            if !(*self.zone.get()).has_enough_size(size) {
                self.grow(size);
            }
            
            let ptr = (*self.zone.get()).alloc(size) as *mut T;
            ptr::write(ptr, v);
            &*ptr
        }
    }
    

    pub fn destroy(&self) {
        if self.destructed.get() {
            return;
        }
        
        self.destructed.set(true);
        unsafe {
            let mut zone = self.head.get();
            loop {
                let next = (*zone).next();
                (*zone).destroy();
                if next.is_null() {
                    break;
                }
                zone = next;
            }
        }
    }

    #[inline]
    fn create_zone(size: usize) -> *const ZoneHeap {
        unsafe {
            let addr = VirtualHeapAllocator::map(ptr::null_mut() as *mut c_void, size as u64, prot::READ | prot::WRITE,
                                                 flags::ANONYMOUS | flags::PRIVATE,
                                                 types::RESERVE);
            match addr {
                Ok(p) => ZoneHeap::new(p as *mut Byte, PAGE_SIZE),
                Err(e) => {
                    panic!("{}", e);
                }
            }
        }
    }

    #[inline]
    fn grow(&self, size: usize) {
        let allocation_size = if size > PAGE_SIZE {
            size * 2
        } else {
            PAGE_SIZE
        };
        
        unsafe {
            let zone = ZoneAllocator::create_zone(size);
            (*self.zone.get()).set_next(zone);
            self.zone.set(zone);
        }
    }
}


impl Drop for ZoneAllocator {
    fn drop(&mut self) {
        self.destroy();
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::mem;

    struct Point {
        x: i32,
        y: i32
    }

    impl ZoneObject<Point> for Point {}

    impl Point {
        pub fn new() -> Point {
            Point {x: 5, y: 5}
        }
    }
    
    #[test]
    fn test() {
        let z = ZoneAllocator::new();
        for x in 0..1000 {
            let p = z.alloc(Point::new());
            assert!(p.x == 5);
            assert!(p.y == 5);
        }
        z.destroy();
    }
}
