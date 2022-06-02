use libc::{size_t, c_void, c_int};

#[link(name = "hoard", kind = "static")]
extern "C" {
    pub fn malloc(size: size_t) -> *mut c_void;
    pub fn calloc(n: size_t, size: size_t) -> *mut c_void;
    pub fn posix_memalign(ptr: *mut *mut c_void, alignment: size_t, size: size_t) -> c_int;
    pub fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    pub fn free(p: *mut c_void);
    pub fn malloc_usable_size(p: *mut c_void) -> size_t;
}
