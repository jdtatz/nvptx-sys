use core::alloc::{GlobalAlloc, Layout};

extern "C" {
    #[link_name = "malloc"]
    pub fn malloc(size: i64) -> *mut u8;
    #[link_name = "free"]
    pub fn free(ptr: *mut u8);
}

struct CudaAllocator;

unsafe impl GlobalAlloc for CudaAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        malloc(layout.size() as i64)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr)
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: CudaAllocator = CudaAllocator;

#[alloc_error_handler]
fn handle_alloc_error(_layout: Layout) -> ! {
    panic!("alloc error")
}
