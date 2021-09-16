use crate::syscall::{free, malloc};
use core::alloc::{GlobalAlloc, Layout};

pub struct CudaSysAllocator;

unsafe impl GlobalAlloc for CudaSysAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { malloc(layout.size()) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe { free(ptr) }
    }
}

#[cfg(feature = "unstable-allocator-api")]
unsafe impl core::alloc::Allocator for CudaSysAllocator {
    fn allocate(
        &self,
        layout: Layout,
    ) -> Result<core::ptr::NonNull<[u8]>, core::alloc::AllocError> {
        core::ptr::NonNull::new(core::ptr::slice_from_raw_parts_mut(
            unsafe { malloc(layout.size()) },
            layout.size(),
        ))
        .ok_or(core::alloc::AllocError)
    }

    unsafe fn deallocate(&self, ptr: core::ptr::NonNull<u8>, _layout: Layout) {
        unsafe { free(ptr.as_ptr()) }
    }
}

#[cfg(feature = "global-allocator")]
#[global_allocator]
static GLOBAL_ALLOCATOR: CudaSysAllocator = CudaSysAllocator;
