use core::panic::PanicInfo;

extern "C" {
    #[link_name = "__assertfail"]
    pub fn __assertfail(
        message: *const u8,
        file: *const u8,
        line: u32,
        function: *const u8,
        char_size: usize,
    ) -> !;
}

#[panic_handler]
unsafe fn cuda_panic_handler(panic_info: &PanicInfo) -> ! {
    let (file, line) = if let Some(loc) = panic_info.location() {
        (loc.file(), loc.line())
    } else {
        ("unknown\0", 0)
    };

    let message = {
        #[cfg(feature = "noisy-panics")] {
            #[cfg(feature = "alloc")] {
                if let Some(args) = panic_info.message() {
                    let mut msg = alloc::string::String::new();
                    if let Err(_e) = core::fmt::write(&mut msg, *args) {
                        alloc::borrow::Cow::Borrowed("Secondary error when trying to display previous")
                    } else {
                        alloc::borrow::Cow::Owned(msg)
                    }
                } else if let Some(msg) = panic_info.payload().downcast_ref::<&'static str>() {
                    alloc::borrow::Cow::Borrowed(*msg)
                } else if let Some(msg) = panic_info.payload().downcast_ref::<alloc::string::String>() {
                    alloc::borrow::Cow::Borrowed(msg.as_str())
                } else {
                    alloc::borrow::Cow::Borrowed("unknown error")
                }
            }
            #[cfg(not(feature = "alloc"))] {
                if let Some(msg) = panic_info.payload().downcast_ref::<&'static str>() {
                    msg
                } else {
                    "unknown error"
                }
            }
        }
        #[cfg(not(feature = "noisy-panics"))] {
            "panicked!\0"
        }
    };
    // hopefully null terminators are optional
    __assertfail(
        message.as_ptr(),
        file.as_ptr(),
        line,
        b"unknown\0".as_ptr(),
        1
    );
}
