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

//Same deal for MEMHeapHeader itself

#[repr(C)]
pub struct MEMHeapHeader {
    #[doc = "! Tag indicating which type of heap this is"]
    pub tag: MEMHeapTag,
    #[doc = "! Link for list this heap is in"]
    pub link: MEMMemoryLink,
    #[doc = "! List of all child heaps in this heap"]
    pub list: MEMMemoryList,
    #[doc = "! Pointer to start of allocatable memory"]
    pub dataStart: *mut cty::c_void,
    #[doc = "! Pointer to end of allocatable memory"]
    pub dataEnd: *mut cty::c_void,
    #[doc = "! Lock used when MEM_HEAP_FLAG_USE_LOCK is set."]
    pub lock: OSSpinLock,
    #[doc = "! Flags set during heap creation."]
    pub flags: u32,
    pub __unk33: [cty::c_char; 12usize],
}
#[test]
fn bindgen_test_layout_MEMHeapHeader() {
    assert_eq!(
        ::core::mem::size_of::<MEMHeapHeader>(),
        64usize,
        concat!("Size of: ", stringify!(MEMHeapHeader))
    );
    assert_eq!(
        ::core::mem::align_of::<MEMHeapHeader>(),
        16usize,
        concat!("Alignment of ", stringify!(MEMHeapHeader))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<MEMHeapHeader>())).tag as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(MEMHeapHeader),
            "::",
            stringify!(tag)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<MEMHeapHeader>())).link as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(MEMHeapHeader),
            "::",
            stringify!(link)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<MEMHeapHeader>())).list as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(MEMHeapHeader),
            "::",
            stringify!(list)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<MEMHeapHeader>())).dataStart as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(MEMHeapHeader),
            "::",
            stringify!(dataStart)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<MEMHeapHeader>())).dataEnd as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(MEMHeapHeader),
            "::",
            stringify!(dataEnd)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<MEMHeapHeader>())).lock as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(MEMHeapHeader),
            "::",
            stringify!(lock)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<MEMHeapHeader>())).flags as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(MEMHeapHeader),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<MEMHeapHeader>())).__unk33 as *const _ as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(MEMHeapHeader),
            "::",
            stringify!(__unk33)
        )
    );
}

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
