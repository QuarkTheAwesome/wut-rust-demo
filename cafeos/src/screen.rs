//wrapper for OSScreen
//TODO: allow only one OSScreen struct at a time

extern crate alloc;
use alloc::alloc::{alloc_zeroed, dealloc};
use core::alloc::Layout;
use core::convert::TryInto;

use cafeos_sys as ffi;

pub struct OSScreen {
    tv_lyt: Layout,
    tv_buf: *mut u8,
    drc_lyt: Layout,
    drc_buf: *mut u8,
}

impl OSScreen {
    pub fn init() -> Result<Self, i32> {
        unsafe {
            ffi::OSScreenInit();
            let tv_sz: usize  = ffi::OSScreenGetBufferSizeEx(ffi::OSScreenID::SCREEN_TV).try_into().unwrap();
            let drc_sz: usize = ffi::OSScreenGetBufferSizeEx(ffi::OSScreenID::SCREEN_DRC).try_into().unwrap();

            let tv_lyt  = Layout::from_size_align(tv_sz, 0x100).unwrap();
            let drc_lyt = Layout::from_size_align(drc_sz, 0x100).unwrap();

            let tv_buf = alloc_zeroed(tv_lyt);
            let drc_buf = alloc_zeroed(drc_lyt);

            ffi::OSScreenSetBufferEx(ffi::OSScreenID::SCREEN_TV, tv_buf as *mut cty::c_void);
            ffi::OSScreenSetBufferEx(ffi::OSScreenID::SCREEN_DRC, drc_buf as *mut cty::c_void);

            ffi::OSScreenEnableEx(ffi::OSScreenID::SCREEN_TV, 1);
            ffi::OSScreenEnableEx(ffi::OSScreenID::SCREEN_DRC, 1);

            Ok(OSScreen { tv_lyt, tv_buf, drc_lyt, drc_buf })
        }
    }

    pub fn flip(&self) {
        unsafe {
            ffi::OSScreenFlipBuffersEx(ffi::OSScreenID::SCREEN_TV);
            ffi::OSScreenFlipBuffersEx(ffi::OSScreenID::SCREEN_DRC);
        }
    }

    pub fn text(&self, text: &str, x: u32, y: u32) {
        let buf = [text, "\0"].concat();
        unsafe {
            ffi::OSScreenPutFontEx(ffi::OSScreenID::SCREEN_TV, y, x, buf.as_ptr());
            ffi::OSScreenPutFontEx(ffi::OSScreenID::SCREEN_DRC, y, x, buf.as_ptr());
        }
    }
}
impl Drop for OSScreen {
    fn drop(&mut self) {
        unsafe {
            if ffi::ProcUIInForeground() != 0 {
                ffi::OSScreenShutdown();
                dealloc(self.tv_buf, self.tv_lyt);
                dealloc(self.drc_buf, self.drc_lyt);
            }
        }
    }
}
