use crate::syscall::{free, malloc};
use core::alloc::{GlobalAlloc, Layout};

struct CudaAllocator;

unsafe impl GlobalAlloc for CudaAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        malloc(layout.size())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr)
    }
}

#[cfg(feature = "unstable-allocator-api")]
unsafe impl core::alloc::Allocator for CudaAllocator {
    fn allocate(
        &self,
        layout: Layout,
    ) -> Result<core::ptr::NonNull<[u8]>, core::alloc::AllocError> {
        todo!()
    }

    unsafe fn deallocate(&self, ptr: core::ptr::NonNull<u8>, layout: Layout) {
        todo!()
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: CudaAllocator = CudaAllocator;

#[alloc_error_handler]
fn handle_alloc_error(_layout: Layout) -> ! {
    panic!("alloc error")
}
