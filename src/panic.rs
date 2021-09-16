use crate::syscall::__assertfail;
use core::panic::PanicInfo;

#[panic_handler]
unsafe fn cuda_panic_handler(panic_info: &PanicInfo) -> ! {
    let (file, line) = if let Some(loc) = panic_info.location() {
        (loc.file(), loc.line())
    } else {
        unsafe { core::hint::unreachable_unchecked() };
    };

    let message = {
        #[cfg(feature = "noisy-panics")]
        {
            #[cfg(feature = "global-allocator")]
            {
                if let Some(args) = panic_info.message() {
                    if let Some(msg) = args.as_str() {
                        alloc::borrow::Cow::Borrowed(msg)
                    } else {
                        let mut msg = alloc::string::String::new();
                        if let Err(core::fmt::Error) = core::fmt::write(&mut msg, *args) {
                            alloc::borrow::Cow::Borrowed("Failed to create panic message\0")
                        } else {
                            alloc::borrow::Cow::Owned(msg)
                        }
                    }
                } else {
                    unsafe { core::hint::unreachable_unchecked() };
                    // alloc::borrow::Cow::Borrowed("unknown error")
                }
            }
            #[cfg(not(feature = "global-allocator"))]
            {
                if let Some(args) = panic_info.message() {
                    if let Some(msg) = args.as_str() {
                        msg
                    } else {
                        "panicked!\0"
                    }
                } else {
                    unsafe { core::hint::unreachable_unchecked() };
                    // "unknown error"
                }
            }
        }
        #[cfg(not(feature = "noisy-panics"))]
        {
            "panicked!\0"
        }
    };
    // hopefully null terminators are optional
    unsafe {
        __assertfail(
            message.as_ptr(),
            file.as_ptr(),
            line,
            b"unknown\0".as_ptr(),
            1,
        )
    }
}
