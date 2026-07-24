#![allow(unused_imports, dead_code)]

mod chtrie;
mod chtrie_h;

extern "C" {
    fn __transpiler_isa(child: i32, ancestor: i32)
    -> bool;
    fn free(_: *mut ())
    -> ();
    fn __error()
    -> *mut i32;
    fn calloc(__count: u64, __size: u64)
    -> *mut ();
    fn malloc(__size: u64)
    -> *mut ();
    fn __builtin_unreachable()
    -> ();
}
