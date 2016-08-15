#![allow(bad_style)]

#![crate_type = "rlib"]
#![crate_name = "libc"]
#![no_std]

#![feature(staged_api)]
#![unstable(feature = "libc",
            reason = "use `libc` from crates.io",
            issue = "27783")]

pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;

#[repr(u8)]
pub enum c_void {
    __variant1,
    __variant2,
}

pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_char = i8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_long = i64;
pub type c_ulong = u64;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_float = f32;
pub type c_double = f64;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type intmax_t = i64;
pub type uintmax_t = u64;
pub type wchar_t = i32;

pub type size_t = usize;
pub type ptrdiff_t = isize;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type ssize_t = isize;

extern {
    pub fn calloc(nobj: size_t, size: size_t) -> *mut c_void;
    pub fn malloc(size: size_t) -> *mut c_void;
    pub fn realloc(p: *mut c_void, size: size_t) -> *mut c_void;
    pub fn free(p: *mut c_void);
    pub fn abort() -> !;
}
