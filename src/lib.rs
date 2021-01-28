use libc::{size_t, c_void};

#[link(name = "hoard", kind = "static")]
extern "C" {
    pub fn calloc(n: size_t, size: size_t) -> *mut c_void;
    pub fn free(p: *mut c_void);
    pub fn malloc_usable_size(p: *mut c_void) -> size_t;
}