use hoard_sys::{calloc, free, malloc, malloc_usable_size, posix_memalign, realloc};

#[test]
fn allocation_round_trip() {
    unsafe {
        // The bindings must resolve to Hoard, not libc's identically named API.
        assert_ne!(malloc as *const (), libc::malloc as *const ());
        for size in [1, 24, 40, 1024, 65536, 1024 * 1024] {
            let ptr = malloc(size).cast::<u8>();
            assert!(!ptr.is_null());
            assert_eq!(ptr as usize % std::mem::align_of::<libc::max_align_t>(), 0);
            assert!(malloc_usable_size(ptr.cast()) >= size);
            ptr.write_bytes(0x5a, size);
            let grown = realloc(ptr.cast(), size * 2).cast::<u8>();
            assert!(!grown.is_null());
            assert!(std::slice::from_raw_parts(grown, size)
                .iter()
                .all(|&b| b == 0x5a));
            free(grown.cast());
        }
        let ptr = calloc(32, 8).cast::<u8>();
        assert!(!ptr.is_null());
        assert!(std::slice::from_raw_parts(ptr, 256).iter().all(|&b| b == 0));
        assert!(realloc(ptr.cast(), 0).is_null());
        free(std::ptr::null_mut());
        let ptr = realloc(std::ptr::null_mut(), 64);
        assert!(!ptr.is_null());
        assert!(realloc(ptr, usize::MAX).is_null());
        free(ptr);
        assert!(malloc(usize::MAX).is_null());
        assert!(calloc(usize::MAX, 2).is_null());
    }
}

#[test]
fn aligned_allocations() {
    unsafe {
        for alignment in [8, 16, 64, 4096] {
            let mut ptr = std::ptr::null_mut();
            assert_eq!(posix_memalign(&mut ptr, alignment, 257), 0);
            assert!(!ptr.is_null());
            assert_eq!(ptr as usize % alignment, 0);
            assert!(malloc_usable_size(ptr) >= 257);
            ptr.cast::<u8>().write_bytes(0xa5, 257);
            let grown = realloc(ptr, 16384).cast::<u8>();
            assert!(!grown.is_null());
            assert!(std::slice::from_raw_parts(grown, 257)
                .iter()
                .all(|&b| b == 0xa5));
            free(grown.cast());
        }
        let sentinel = std::ptr::NonNull::<u8>::dangling().as_ptr().cast();
        let mut ptr = sentinel;
        *libc::__errno_location() = libc::EINTR;
        assert_eq!(posix_memalign(&mut ptr, 3, 128), libc::EINVAL);
        assert_eq!(ptr, sentinel);
        assert_eq!(posix_memalign(&mut ptr, 64, usize::MAX), libc::ENOMEM);
        assert_eq!(ptr, sentinel);
        assert_eq!(
            posix_memalign(&mut ptr, 1usize << (usize::BITS - 1), 1),
            libc::ENOMEM
        );
        assert_eq!(ptr, sentinel);
        assert_eq!(*libc::__errno_location(), libc::EINTR);
    }
}

#[test]
fn reallocation_preserves_alignment_and_contents() {
    unsafe {
        // Keep allocations live to exercise consecutive objects in size classes
        // whose unrounded sizes would only guarantee eight-byte alignment.
        let mut allocations = Vec::new();
        for _ in 0..32 {
            let ptr = realloc(std::ptr::null_mut(), 1).cast::<u8>();
            assert!(!ptr.is_null());
            ptr.write(0x5a);
            let grown = realloc(ptr.cast(), 40).cast::<u8>();
            assert!(!grown.is_null());
            assert_eq!(
                grown as usize % std::mem::align_of::<libc::max_align_t>(),
                0
            );
            assert_eq!(grown.read(), 0x5a);
            grown.write_bytes(0xa5, 40);
            let shrunk = realloc(grown.cast(), 24).cast::<u8>();
            assert!(!shrunk.is_null());
            assert!(std::slice::from_raw_parts(shrunk, 24)
                .iter()
                .all(|&b| b == 0xa5));
            *libc::__errno_location() = 0;
            assert!(realloc(shrunk.cast(), usize::MAX).is_null());
            assert_eq!(*libc::__errno_location(), libc::ENOMEM);
            assert!(std::slice::from_raw_parts(shrunk, 24)
                .iter()
                .all(|&b| b == 0xa5));
            allocations.push(shrunk);
        }
        for ptr in allocations {
            free(ptr.cast());
        }
    }
}

#[test]
fn allocations_survive_the_allocating_thread() {
    let threads: Vec<_> = (0..8)
        .map(|_| {
            std::thread::spawn(|| unsafe {
                let ptr = malloc(4096).cast::<u8>();
                assert!(!ptr.is_null());
                ptr.write_bytes(0x7b, 4096);
                ptr as usize
            })
        })
        .collect();
    for thread in threads {
        let ptr = thread.join().unwrap() as *mut u8;
        unsafe {
            assert!(std::slice::from_raw_parts(ptr, 4096)
                .iter()
                .all(|&b| b == 0x7b));
            free(ptr.cast());
        }
    }
}
