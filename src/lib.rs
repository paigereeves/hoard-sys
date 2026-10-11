//! Bindings to Hoard. Pointers must be freed or reallocated with this
//! crate, never with libc or Rust's default allocator.

use libc::{c_int, c_void, size_t};
use std::{mem, ptr};

#[link(name = "hoard", kind = "static")]
extern "C" {
    fn xxmemalign(alignment: size_t, size: size_t) -> *mut c_void;
    fn xxrealloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn xxfree(ptr: *mut c_void);
    fn xxmalloc_usable_size(ptr: *mut c_void) -> size_t;
}

// Hoard adds alignment padding without checking for overflow.
unsafe fn allocate(alignment: usize, size: usize) -> *mut c_void {
    if alignment > isize::MAX as usize || size > isize::MAX as usize - alignment {
        *libc::__errno_location() = libc::ENOMEM;
        return ptr::null_mut();
    }
    let allocation = xxmemalign(alignment, size.max(1));
    if allocation.is_null() {
        *libc::__errno_location() = libc::ENOMEM;
    }
    allocation
}

/// Allocates memory aligned for any fundamental C type.
///
/// # Safety
/// The returned allocation must be managed with this crate's functions.
/// The returned memory is uninitialized and must be initialized before reading.
pub unsafe extern "C" fn malloc(size: size_t) -> *mut c_void {
    // Hoard's raw xxmalloc only guarantees eight-byte alignment.
    allocate(mem::align_of::<libc::max_align_t>(), size)
}

/// Allocates zero-initialized memory for `n` elements of `size` bytes.
///
/// # Safety
/// The returned allocation must be managed with this crate's functions.
pub unsafe extern "C" fn calloc(n: size_t, size: size_t) -> *mut c_void {
    let total = match n.checked_mul(size) {
        Some(total) => total,
        None => {
            *libc::__errno_location() = libc::ENOMEM;
            return ptr::null_mut();
        }
    };
    let allocation = malloc(total);
    if !allocation.is_null() {
        allocation.cast::<u8>().write_bytes(0, total);
    }
    allocation
}

/// Allocates aligned memory, returning zero or a POSIX error code.
/// Leaves `errno` and the output pointer unchanged on failure.
///
/// # Safety
/// `out` must be valid for writing a pointer. The returned allocation must
/// be managed with this crate's functions.
pub unsafe extern "C" fn posix_memalign(
    out: *mut *mut c_void,
    alignment: size_t,
    size: size_t,
) -> c_int {
    if alignment < mem::size_of::<*mut c_void>() || !alignment.is_power_of_two() {
        return libc::EINVAL;
    }
    let saved_errno = *libc::__errno_location();
    let allocation = allocate(alignment, size);
    *libc::__errno_location() = saved_errno;
    if allocation.is_null() {
        return libc::ENOMEM;
    }
    out.write(allocation);
    0
}

/// Resizes an allocation, preserving its contents up to the smaller size.
/// A null pointer allocates; a zero size frees a non-null pointer.
/// Failure leaves the original allocation intact.
///
/// # Safety
/// `allocation` must be null or a live allocation from this crate. On success,
/// use only the returned pointer to access the allocation.
pub unsafe extern "C" fn realloc(allocation: *mut c_void, size: size_t) -> *mut c_void {
    if allocation.is_null() {
        return malloc(size);
    }
    if size == 0 {
        free(allocation);
        return ptr::null_mut();
    }
    let alignment = mem::align_of::<libc::max_align_t>();
    if size > isize::MAX as usize - alignment {
        *libc::__errno_location() = libc::ENOMEM;
        return ptr::null_mut();
    }
    // Like xxmemalign's fundamental-alignment path, round the request so
    // xxrealloc's raw xxmalloc allocation uses an aligned size class.
    let rounded_size = (size + alignment - 1) & !(alignment - 1);
    let replacement = xxrealloc(allocation, rounded_size);
    if replacement.is_null() {
        *libc::__errno_location() = libc::ENOMEM;
    }
    replacement
}

/// Frees an allocation; a null pointer is ignored.
///
/// # Safety
/// `allocation` must be null or a live allocation from this crate and must
/// not be accessed after this call.
pub unsafe extern "C" fn free(allocation: *mut c_void) {
    xxfree(allocation);
}

/// Returns the usable allocation size, or zero for a null pointer.
///
/// # Safety
/// `allocation` must be null or a live allocation from this crate.
pub unsafe extern "C" fn malloc_usable_size(allocation: *mut c_void) -> size_t {
    if allocation.is_null() {
        0
    } else {
        xxmalloc_usable_size(allocation)
    }
}
