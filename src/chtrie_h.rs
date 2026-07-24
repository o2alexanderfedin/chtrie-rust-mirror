use super::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct ChtrieEdge {
    pub(crate) next: *mut ChtrieEdge,
    pub(crate) from: i32,
    pub(crate) sym: i32,
    pub(crate) to: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Chtrie {
    pub(crate) etab: *mut *mut ChtrieEdge,
    pub(crate) idxpool: *mut i32,
    pub(crate) idxptr: *mut i32,
    pub(crate) idxmax: i32,
    pub(crate) maxn: i32,
    pub(crate) alphsz: i32,
    pub(crate) ecap: i32,
}
