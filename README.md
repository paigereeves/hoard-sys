# hoard-sys
Rust FFI bindings to [Hoard](https://github.com/emeryberger/Hoard).

Building requires the Hoard and Heap-Layers submodules (`git submodule update
--init --recursive`) and a C++20 compiler. Linux targets are supported; `cc`
compiles the sources directly and honors its standard `CXX`, `CXXFLAGS`, and
`AR` settings, including target-specific overrides for cross compilation.

The Rust API retains `malloc`, `calloc`, `posix_memalign`, `realloc`, `free`,
and `malloc_usable_size`. Private FFI declarations link directly to Hoard's
`xx*` C symbols; alignment, overflow checks, and POSIX return conventions are
handled in Rust. These functions allocate from Hoard without replacing libc's
allocation functions.
Only pass Hoard allocations to this crate's free, realloc, and size functions;
do not mix them with libc or Rust's default allocator.

The build includes Hoard's Unix thread lifecycle implementation, which
interposes `pthread_create` and `pthread_exit` to manage per-thread heaps.

Run `cargo test` and `cargo test --release` to exercise the allocation bindings.
