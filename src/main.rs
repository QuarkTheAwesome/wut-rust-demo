#![no_std]
#![feature(start)]

use cafeos as _;
use cafeos::screen::OSScreen;

unsafe extern "C" fn save_callback() {
    cafeos_sys::OSSavesDone_ReadyToRelease();
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        cafeos_sys::ProcUIInit(Some(save_callback));
        cafeos_sys::OSReport("owo\0".as_ptr() as *const cty::c_char);
    }

    let screen = OSScreen::init().unwrap();

    loop {
        unsafe {
            let status = cafeos_sys::ProcUIProcessMessages(1);
            match status {
                cafeos_sys::ProcUIStatus::PROCUI_STATUS_IN_FOREGROUND => {}
                cafeos_sys::ProcUIStatus::PROCUI_STATUS_RELEASE_FOREGROUND => {
                    cafeos_sys::ProcUIDrawDoneRelease();
                    continue;
                }
                cafeos_sys::ProcUIStatus::PROCUI_STATUS_EXITING => { break; }
                _ => { continue; }
            }
        }

        screen.text("hello world!", 0, 0);
        screen.flip();
    }


    unsafe {
        cafeos_sys::ProcUIShutdown();
        //cafeos_sys::OSFatal("oh dear\0".as_ptr() as *const cty::c_char);
    }

    0
}
