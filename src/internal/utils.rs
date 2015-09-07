///!
///!
///!


pub type Byte = u8;

pub const ONE_BYTE: i32 = 1024;
pub const PAGE_SIZE: usize = kb!(4);

#[cfg(target_arch="x86_x64")]
pub type Pointer = u64;
#[cfg(target_arch="x86")]
pub type Pointer = u32;
