extern crate alloc;
use arrayvec::{Array, ArrayString};
use core::{
    alloc::{GlobalAlloc, Layout},
    cmp,
    ops::Deref,
    panic::PanicInfo,
};
use cty::{c_char, c_void};

// Default allocator implementation
pub struct MEMDefaultHeapAllocator;

unsafe impl GlobalAlloc for MEMDefaultHeapAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        cafeos_sys::MEMAllocFromDefaultHeapEx.unwrap()(layout.size() as u32, cmp::max(layout.align() as i32, 4))
            as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        cafeos_sys::MEMFreeToDefaultHeap.unwrap()(ptr as *mut c_void);
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
        cafeos_sys::OSPanic(
            filename.as_ptr() as *const c_char,
            line as u32,
            message.as_ptr() as *const c_char,
        );
    };
    loop {}
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    let message = ArrayCString::<[_; 32]>::from("alloc_error");
    unsafe {
        cafeos_sys::OSFatal(message.as_ptr() as *const c_char);
    }
    loop {}
}
