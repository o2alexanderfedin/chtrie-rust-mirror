use super::*;
use crate::chtrie_h::{Chtrie, ChtrieEdge};

/// Allocate a trie with at most `n` nodes, and the alphabet size `m`.
///
///If `n` or `m` is less than 1, they will be regulated to 1.
///
///Nodes in the trie are indexed by non-negative integers less than `n`.
///The root node is indexed by 0.
///Symbols are non-negative integers less than `m`.
///
///Upon success, return a pointer to the trie.
///Otherwise, return `NULL` and set `errno`.
pub(crate) extern "C" fn chtrie_alloc(mut n: u64, mut m: u64) -> *mut Chtrie {
    let mut tr: *mut Chtrie = core::ptr::null_mut();
    let mut __state: i32 = 0;
    loop {
        if __state == 1 {
            break;
        }
        '__s1: {
            match __state {
                0 => {
                    __state = 5;
                }
                2 => {
                    unsafe { free(unsafe { (*tr).etab } as *mut ()) };
                    __state = 28;
                }
                3 => {
                    unsafe { free(tr as *mut ()) };
                    __state = 29;
                }
                4 => {
                    return 0 as *mut () as *mut Chtrie;
                }
                5 => {
                    if n < 1 as u64 {
                        __state = 7;
                    } else {
                        __state = 6;
                    }
                }
                6 => {
                    if m < 1 as u64 {
                        __state = 9;
                    } else {
                        __state = 8;
                    }
                }
                7 => {
                    n = 1 as u64;
                    __state = 6;
                }
                8 => {
                    if n > 2147483647 as u64 || m > 2147483647 as u64 {
                        __state = 11;
                    } else {
                        __state = 10;
                    }
                }
                9 => {
                    m = 1 as u64;
                    __state = 8;
                }
                10 => {
                    if if (2147483647 as u64) < -1i32 as u64 {
                        2147483647 as u64
                    } else {
                        -1i32 as u64
                    }
                    .wrapping_sub(n.wrapping_sub(1 as u64))
                        < n.wrapping_sub(1 as u64) / 3 as u64
                    {
                        __state = 14;
                    } else {
                        __state = 13;
                    }
                }
                11 => {
                    unsafe { *unsafe { __error() } = 34 };
                    __state = 12;
                }
                12 => {
                    __state = 4;
                }
                13 => {
                    if ({
                        tr = unsafe { calloc(1 as u64, core::mem::size_of::<Chtrie>() as u64) }
                            as *mut Chtrie;
                        tr
                    })
                    .is_null() as i32
                        != 0
                    {
                        __state = 17;
                    } else {
                        __state = 16;
                    }
                }
                14 => {
                    unsafe { *unsafe { __error() } = 34 };
                    __state = 15;
                }
                15 => {
                    __state = 4;
                }
                16 => {
                    unsafe { (*tr).maxn = n as i32 };
                    __state = 18;
                }
                17 => {
                    __state = 4;
                }
                18 => {
                    unsafe { (*tr).alphsz = m as i32 };
                    __state = 19;
                }
                19 => {
                    unsafe {
                        (*tr).ecap = n
                            .wrapping_sub(1 as u64)
                            .wrapping_add(n.wrapping_sub(1 as u64) / 3 as u64)
                            as i32
                    };
                    __state = 20;
                }
                20 => {
                    if ({
                        let __v = unsafe {
                            calloc(
                                unsafe { (*tr).ecap } as u64,
                                core::mem::size_of::<*mut ChtrieEdge>() as u64,
                            )
                        } as *mut *mut ChtrieEdge;
                        unsafe { (*tr).etab = __v };
                        __v
                    })
                    .is_null() as i32
                        != 0
                    {
                        __state = 22;
                    } else {
                        __state = 21;
                    }
                }
                21 => {
                    if ({
                        let __v =
                            unsafe { calloc(n, core::mem::size_of::<i32>() as u64) } as *mut i32;
                        unsafe { (*tr).idxpool = __v };
                        __v
                    })
                    .is_null() as i32
                        != 0
                    {
                        __state = 24;
                    } else {
                        __state = 23;
                    }
                }
                22 => {
                    __state = 3;
                }
                23 => {
                    unsafe { (*tr).idxmax = 1 };
                    __state = 25;
                }
                24 => {
                    __state = 2;
                }
                25 => {
                    unsafe { (*tr).idxptr = unsafe { (*tr).idxpool } };
                    __state = 26;
                }
                26 => {
                    return tr;
                }
                27 => {
                    __state = 2;
                }
                28 => {
                    __state = 3;
                }
                29 => {
                    __state = 4;
                }
                _ => {}
            }
        }
    }
    unreachable!();
}

/// Walk from one node to its child.
///
///If the child didn't exist and `creat` is non-zero,
///a new node will be created.
///
///Upon the child is found or created, return the index of the child.
///Otherwise, return -1.
///If `creat` is non-zero and this routine fails, `errno` will be set.
pub(crate) extern "C" fn chtrie_walk(tr: &mut Chtrie, from: i32, sym: i32, creat: i32) -> i32 {
    let mut p: *mut ChtrieEdge = core::ptr::null_mut();
    let mut h: u64 = 0 as u64;
    h = (from as u64)
        .wrapping_mul((*tr).alphsz as u64)
        .wrapping_add(sym as u64);
    h %= (*tr).ecap as u64;
    {
        p = unsafe { *(*tr).etab.add(h as usize) };
        '__b2: loop {
            if !(!(p).is_null()) {
                break '__b2;
            }
            '__c2: loop {
                if unsafe { (*p).from } == from && unsafe { (*p).sym } == sym {
                    return unsafe { (*p).to };
                }
                break '__c2;
            }
            p = unsafe { (*p).next };
        }
    }
    if creat != 0 {
        if (*tr).idxptr == (*tr).idxpool && (*tr).idxmax >= (*tr).maxn {
            unsafe { *unsafe { __error() } = 12 };
            return -1;
        }
        if ({
            p = unsafe { malloc(core::mem::size_of::<ChtrieEdge>() as u64) } as *mut ChtrieEdge;
            p
        })
        .is_null() as i32
            != 0
        {
            return -1;
        }
        unsafe { (*p).next = unsafe { *(*tr).etab.add(h as usize) } };
        unsafe { *(*tr).etab.add(h as usize) = p };
        unsafe { (*p).from = from };
        unsafe { (*p).sym = sym };
        if (*tr).idxptr != (*tr).idxpool {
            unsafe {
                (*p).to = unsafe {
                    *{
                        let __p = &mut (*tr).idxptr;
                        *__p = unsafe { (*__p).offset(-1) };
                        *__p
                    }
                }
            };
        } else {
            unsafe {
                (*p).to = {
                    let __p = &mut (*tr).idxmax;
                    let __t = *__p;
                    *__p += 1;
                    __t
                }
            };
        }
        return unsafe { (*p).to };
    }
    return -1;
}

/// Delete a child node.
///
///If the child doesn't exist, the trie shall be left unchanged.
pub(crate) extern "C" fn chtrie_del(tr: &mut Chtrie, from: i32, sym: i32) -> () {
    let mut p: *mut ChtrieEdge = core::ptr::null_mut();
    let mut q: *mut ChtrieEdge = core::ptr::null_mut();
    let mut h: u64 = 0 as u64;
    h = (from as u64)
        .wrapping_mul((*tr).alphsz as u64)
        .wrapping_add(sym as u64);
    h %= (*tr).ecap as u64;
    {
        {
            p = unsafe { *(*tr).etab.add(h as usize) };
            q = 0 as *mut () as *mut ChtrieEdge
        };
        '__b3: loop {
            if !(!(p).is_null()) {
                break '__b3;
            }
            '__c3: loop {
                if unsafe { (*p).from } == from && unsafe { (*p).sym } == sym {
                    break '__b3;
                }
                break '__c3;
            }
            {
                q = p;
                p = unsafe { (*p).next }
            };
        }
    }
    if (p).is_null() as i32 != 0 {
        return;
    }
    if !(q).is_null() {
        unsafe { (*q).next = unsafe { (*p).next } };
    } else {
        unsafe { *(*tr).etab.add(h as usize) = 0 as *mut () as *mut ChtrieEdge };
    }
    unsafe {
        *{
            let __p = &mut (*tr).idxptr;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        } = unsafe { (*p).to }
    };
    unsafe { free(p as *mut ()) };
}

/// free the trie
pub(crate) extern "C" fn chtrie_free(tr: *mut Chtrie) -> () {
    let mut p: *mut ChtrieEdge = core::ptr::null_mut();
    let mut q: *mut ChtrieEdge = core::ptr::null_mut();
    let mut i: i32 = 0;
    {
        i = 0;
        '__b4: loop {
            if !(i < unsafe { (*tr).ecap }) {
                break '__b4;
            }
            '__c4: loop {
                p = unsafe { *unsafe { (*tr).etab.offset(i as isize) } };
                while !(p).is_null() {
                    q = unsafe { (*p).next };
                    unsafe { free(p as *mut ()) };
                    p = q;
                }
                break '__c4;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
    unsafe { free(unsafe { (*tr).etab } as *mut ()) };
    unsafe { free(unsafe { (*tr).idxpool } as *mut ()) };
    unsafe { free(tr as *mut ()) };
}
