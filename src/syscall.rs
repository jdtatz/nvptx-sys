extern "C" {
    #[link_name = "vprintf"]
    pub fn vprintf(format: *const u8, va_list: *mut ::core::ffi::c_void) -> i32;
    #[link_name = "__assertfail"]
    pub fn __assertfail(
        message: *const u8,
        file: *const u8,
        line: u32,
        function: *const u8,
        char_size: usize,
    ) -> !;
    /// The returned pointer is guaranteed to be aligned to a 16-byte boundary,
    /// but the actual implementation usually aligns to 128 or 256-bytes boundaries
    #[link_name = "malloc"]
    pub fn malloc(size: usize) -> *mut u8;
    #[link_name = "free"]
    pub fn free(ptr: *mut u8);
}
