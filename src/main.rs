#![no_std]
#![feature(start)]

use cafeos as _;

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        if cafeos_sys::OSIsDebuggerPresent() != 0 {
            1
        } else {
            0
        }
    }
}
