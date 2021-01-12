#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

// Unfortunately OSSpinLock is included in structures which have packed
// attribute such as MEMHeapHeader, Rust cannot combine align and packed on the
// same type, so instead we will define OSSpinLock without align to work around
// this issue. Not ideal but what can we do eh.
// error[E0588]: packed type cannot transitively contain a `#[repr(align)]` type

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OSSpinLock {
    pub owner: u32,
    pub __unk31: [cty::c_char; 4usize],
    pub recursion: u32,
    pub __unk32: [cty::c_char; 4usize],
}
#[test]
fn bindgen_test_layout_OSSpinLock() {
    assert_eq!(
        ::core::mem::size_of::<OSSpinLock>(),
        16usize,
        concat!("Size of: ", stringify!(OSSpinLock))
    );
    assert_eq!(
        ::core::mem::align_of::<OSSpinLock>(),
        4usize, //16usize,
        concat!("Alignment of ", stringify!(OSSpinLock))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<OSSpinLock>())).owner as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(OSSpinLock),
            "::",
            stringify!(owner)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<OSSpinLock>())).__unk31 as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(OSSpinLock),
            "::",
            stringify!(__unk31)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<OSSpinLock>())).recursion as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(OSSpinLock),
            "::",
            stringify!(recursion)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<OSSpinLock>())).__unk32 as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(OSSpinLock),
            "::",
            stringify!(__unk32)
        )
    );
}

include!("sys.rs");
