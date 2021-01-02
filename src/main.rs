#![no_std]
#![feature(start)]
#![feature(alloc_error_handler)]

extern crate alloc;
extern crate libc;
use libc::{c_char, c_int};
use arrayvec::{Array, ArrayString};

use core::{
    alloc::{GlobalAlloc, Layout},
    cmp,
    ops::Deref,
    panic::PanicInfo,
};

pub struct MEMDefaultHeapAllocator;

#[link(name = "wut")]
extern {
    fn OSIsDebuggerPresent() -> bool;
    static mut MEMAllocFromDefaultHeapEx: Option<extern "C" fn(u32, i32) -> *mut u8>;
    static mut MEMFreeToDefaultHeap: Option<extern "C" fn(*mut u8)>;
    static mut OSPanic: Option<extern "C" fn(*const c_char, i32, *const c_char)>;
    static mut OSFatal: Option<extern "C" fn(*const c_char)>;
}

#[global_allocator]
static GLOBAL_ALLOCATOR: MEMDefaultHeapAllocator = MEMDefaultHeapAllocator;

unsafe impl GlobalAlloc for MEMDefaultHeapAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        MEMAllocFromDefaultHeapEx.unwrap()(layout.size() as u32, layout.align() as i32)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        MEMFreeToDefaultHeap.unwrap()(ptr);
    }
}

fn str_truncate_valid(s: &str, mut mid: usize) -> &str {
    loop {
        if let Some(res) = s.get(..mid) {
            return res;
        }
        mid -= 1;
    }
}

#[derive(Debug, PartialEq)]
struct ArrayCString<A: Array<Item = u8> + Copy> {
    inner: ArrayString<A>,
}

impl<S: AsRef<str>, A: Array<Item = u8> + Copy> From<S> for ArrayCString<A> {
    fn from(s: S) -> Self {
        let s = s.as_ref();
        let len = cmp::min(s.len(), A::CAPACITY - 1);
        let mut result = Self {
            inner: ArrayString::from(str_truncate_valid(s, len)).unwrap(),
        };
        result.inner.push('\0');
        result
    }
}

impl<A: Array<Item = u8> + Copy> Deref for ArrayCString<A> {
    type Target = str;

    fn deref(&self) -> &str {
        self.inner.as_str()
    }
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    let payload = info.payload();
    let message = if let Some(s) = payload.downcast_ref::<&str>() {
        s
    } else {
        "Unhandled rust panic payload!"
    };
    let (filename, line) = if let Some(loc) = info.location() {
        (loc.file(), loc.line())
    } else {
        ("unknown.rs", 0)
    };
    // Copy the message and filename to the stack in order to safely add
    // a terminating nul character (since rust strings don't come with one).
    let message = ArrayCString::<[_; 256]>::from(message);
    let filename = ArrayCString::<[_; 128]>::from(filename);
    unsafe {
        OSPanic.unwrap()(
            filename.as_ptr() as *const c_char,
            line as c_int,
            message.as_ptr() as *const c_char,
        );
    };
    loop {}
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    let message = ArrayCString::<[_; 32]>::from("alloc_error");
    unsafe {
        OSFatal.unwrap()(
            message.as_ptr() as *const c_char
        );
    }
    loop {}
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        if OSIsDebuggerPresent() {
            1
        } else {
            0
        }
    }
}
