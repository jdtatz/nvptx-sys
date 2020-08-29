#![no_std]
#![feature(core_intrinsics, asm, global_asm, link_llvm_intrinsics)]
#![cfg_attr(feature = "alloc", feature(alloc_error_handler))]
#![cfg_attr(all(feature = "panic", feature = "alloc"), feature(panic_info_message))]
#![allow(non_camel_case_types)]

#[cfg(feature = "alloc")]
extern crate alloc;

/*
https://docs.nvidia.com/cuda/parallel-thread-execution/index.htm
https://docs.nvidia.com/cuda/ptx-writers-guide-to-interoperability/index.html
*/

#[cfg(feature = "panic")]
mod panic;
#[cfg(feature = "alloc")]
mod allocator;
mod float;
pub use crate::float::*;
use core::ptr::NonNull;


extern "C" {
    #[link_name = "llvm.nvvm.read.ptx.sreg.tid.x"]
    fn read_ptx_sreg_tid_x() -> i32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.tid.y"]
    fn read_ptx_sreg_tid_y() -> i32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.tid.z"]
    fn read_ptx_sreg_tid_z() -> i32;

    #[link_name = "llvm.nvvm.read.ptx.sreg.ntid.x"]
    fn read_ptx_sreg_ntid_x() -> i32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.ntid.y"]
    fn read_ptx_sreg_ntid_y() -> i32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.ntid.z"]
    fn read_ptx_sreg_ntid_z() -> i32;

    #[link_name = "llvm.nvvm.read.ptx.sreg.ctaid.x"]
    fn read_ptx_sreg_ctaid_x() -> i32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.ctaid.y"]
    fn read_ptx_sreg_ctaid_y() -> i32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.ctaid.z"]
    fn read_ptx_sreg_ctaid_z() -> i32;

    #[link_name = "llvm.nvvm.read.ptx.sreg.nctaid.x"]
    fn read_ptx_sreg_nctaid_x() -> i32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.nctaid.y"]
    fn read_ptx_sreg_nctaid_y() -> i32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.nctaid.z"]
    fn read_ptx_sreg_nctaid_z() -> i32;

    #[link_name = "llvm.nvvm.atomic.load.add.f32.p0f32"]
    pub fn atomic_load_add_f32(address: *mut f32, val: f32) -> f32;

    #[link_name = "llvm.nvvm.read.ptx.sreg.laneid"]
    fn read_nvvm_read_ptx_sreg_laneid() -> i32;

    #[link_name = "llvm.nvvm.shfl.sync.idx.i32"]
    fn shfl_sync_i32(mask: u32, val: i32, src_lane: u32, packing: u32) -> i32;
    #[link_name = "llvm.nvvm.shfl.sync.down.i32"]
    fn shfl_down_sync_i32(mask: u32, val: i32, delta: u32, packing: u32) -> i32;
    #[link_name = "llvm.nvvm.shfl.sync.up.i32"]
    fn shfl_up_sync_i32(mask: u32, val: i32, delta: u32, packing: u32) -> i32;
    #[link_name = "llvm.nvvm.shfl.sync.bfly.i32"]
    fn shfl_bfly_sync_i32(mask: u32, val: i32, lane_mask: u32, packing: u32) -> i32;

    #[link_name = "llvm.nvvm.shfl.sync.idx.f32"]
    fn shfl_sync_f32(mask: u32, val: f32, src_lane: u32, packing: u32) -> f32;
    #[link_name = "llvm.nvvm.shfl.sync.down.f32"]
    fn shfl_down_sync_f32(mask: u32, val: f32, delta: u32, packing: u32) -> f32;
    #[link_name = "llvm.nvvm.shfl.sync.up.f32"]
    fn shfl_up_sync_f32(mask: u32, val: f32, delta: u32, packing: u32) -> f32;
    #[link_name = "llvm.nvvm.shfl.sync.bfly.f32"]
    fn shfl_bfly_sync_f32(mask: u32, val: f32, lane_mask: u32, packing: u32) -> f32;

    #[link_name = "llvm.nvvm.barrier0"]
    fn __syncthreads();
    #[link_name = "llvm.nvvm.barrier0.or"]
    fn __syncthreads_or(test: i32) -> i32;
    #[link_name = "llvm.nvvm.barrier0.popc"]
    fn __syncthreads_count(test: i32) -> i32;

    #[link_name = "vprintf"]
    pub fn vprintf(format: *const u8, va_list: *mut u8) -> i32;
    #[link_name = "__assertfail"]
    pub fn __assertfail(
        message: *const u8,
        file: *const u8,
        line: u32,
        function: *const u8,
        char_size: usize,
    ) -> !;
}


pub struct threadIdx {}
pub struct blockIdx {}
pub struct blockDim {}
pub struct gridDim {}

impl threadIdx {
    pub fn x() -> usize {
        unsafe { read_ptx_sreg_tid_x() as usize }
    }
    pub fn y() -> usize {
        unsafe { read_ptx_sreg_tid_y() as usize }
    }
    pub fn z() -> usize {
        unsafe { read_ptx_sreg_tid_z() as usize }
    }
}

impl blockIdx {
    pub fn x() -> usize {
        unsafe { read_ptx_sreg_ctaid_x() as usize }
    }
    pub fn y() -> usize {
        unsafe { read_ptx_sreg_ctaid_y() as usize }
    }
    pub fn z() -> usize {
        unsafe { read_ptx_sreg_ctaid_z() as usize }
    }
}

impl blockDim {
    pub fn x() -> usize {
        unsafe { read_ptx_sreg_ntid_x() as usize }
    }
    pub fn y() -> usize {
        unsafe { read_ptx_sreg_ntid_y() as usize }
    }
    pub fn z() -> usize {
        unsafe { read_ptx_sreg_ntid_z() as usize }
    }
}

impl gridDim {
    pub fn x() -> usize {
        unsafe { read_ptx_sreg_nctaid_x() as usize }
    }
    pub fn y() -> usize {
        unsafe { read_ptx_sreg_nctaid_y() as usize }
    }
    pub fn z() -> usize {
        unsafe { read_ptx_sreg_nctaid_z() as usize }
    }
}

pub fn laneid() -> usize {
    unsafe { read_nvvm_read_ptx_sreg_laneid() as usize }
}

pub fn syncthreads() {
    unsafe {
        // core::intrinsics::atomic_singlethreadfence();
        __syncthreads();
        // core::intrinsics::atomic_singlethreadfence();
    }
}

pub fn syncthreads_or(test: bool) -> bool {
    unsafe {
        // core::intrinsics::atomic_singlethreadfence();
        let b = __syncthreads_or(if test { 1 } else { 0 }) != 0;
        // core::intrinsics::atomic_singlethreadfence();
        b
    }
}

pub fn syncthreads_count(test: bool) -> usize {
    unsafe {
        // core::intrinsics::atomic_singlethreadfence();
        let c = __syncthreads_count(if test { 1 } else { 0 }) as usize;
        // core::intrinsics::atomic_singlethreadfence();
        c
    }
}

pub const ALL_MEMBER_MASK: u32 = 0xffffffff;

pub trait Shuffle {
    fn shfl(self, mask: u32, src_lane: u32) -> Self;
    fn shfl_down(self, mask: u32, delta: u32) -> Self;
    fn shfl_up(self, mask: u32, delta: u32) -> Self;
    fn shfl_bfly(self, mask: u32, lane_mask: u32) -> Self;
}

impl Shuffle for i32 {
    fn shfl(self, mask: u32, src_lane: u32) -> Self {
        unsafe { shfl_sync_i32(mask, self, src_lane, 0x1f) }
    }

    fn shfl_down(self, mask: u32, delta: u32) -> Self {
        unsafe { shfl_down_sync_i32(mask, self, delta, 0x1f) }
    }

    fn shfl_up(self, mask: u32, delta: u32) -> Self {
        unsafe { shfl_up_sync_i32(mask, self, delta, 0) }
    }

    fn shfl_bfly(self, mask: u32, lane_mask: u32) -> Self {
        unsafe { shfl_bfly_sync_i32(mask, self, lane_mask, 0x1f) }
    }
}

impl Shuffle for f32 {
    fn shfl(self, mask: u32, src_lane: u32) -> Self {
        unsafe { shfl_sync_f32(mask, self, src_lane, 0x1f) }
    }

    fn shfl_down(self, mask: u32, delta: u32) -> Self {
        unsafe { shfl_down_sync_f32(mask, self, delta, 0x1f) }
    }

    fn shfl_up(self, mask: u32, delta: u32) -> Self {
        unsafe { shfl_up_sync_f32(mask, self, delta, 0) }
    }

    fn shfl_bfly(self, mask: u32, lane_mask: u32) -> Self {
        unsafe { shfl_bfly_sync_f32(mask, self, lane_mask, 0x1f) }
    }
}

// #[repr(transparent)]
// pub struct AtomicF32 {
//     inner: UnsafeCell<f32>
// }
//
// impl AtomicF32 {
//
// }

global_asm!(".extern .shared .align 16 .b8 dynamic_shared_memory[];");

pub unsafe fn dynamic_shared_memory() -> (*mut (), usize) {
    let mut shared_ptr;
    let mut dyn_mem_size: u32;
    asm!(
        "cvta.shared.u64 {ptr}, dynamic_shared_memory; mov.u32 {sz}, %dynamic_smem_size;",
        ptr = out(reg64) shared_ptr,
        sz = out(reg32) dyn_mem_size,
        options(pure, nomem, nostack, preserves_flags)
    );
    (shared_ptr, dyn_mem_size as usize)
}

pub unsafe fn dynamic_shared_array<T: 'static + Sized + Send + Sync>(n: usize) -> Option<NonNull<[T]>> {
    let mut shared_ptr;
    let mut dyn_mem_size: u32;
    asm!(
        "cvta.shared.u64 {ptr}, dynamic_shared_memory; mov.u32 {sz}, %dynamic_smem_size;",
        ptr = out(reg64) shared_ptr,
        sz = out(reg32) dyn_mem_size,
        options(pure, nomem, nostack, preserves_flags)
    );
    if n * core::mem::size_of::<T>() <= dyn_mem_size as usize {
        Some(NonNull::new_unchecked(core::ptr::slice_from_raw_parts_mut(shared_ptr, n)))
    } else {
        None
    }
}

pub unsafe fn shared<T: 'static + Sized + Send + Sync>() -> *mut T {
    let mut shared_ptr: *mut T;
    asm!(
        "{{ .shared .align {a} .b8 shared_value[{n}]; cvta.shared.u64 {ptr}, shared_value; }}",
        a = const core::mem::align_of::<T>(),
        n = const core::mem::size_of::<T>(),
        ptr = out(reg64) shared_ptr,
        options(nostack, preserves_flags)
    );
    shared_ptr
}

#[macro_export]
macro_rules! printf {
	($fmt:literal, $($args:expr),* $(,)?) => {
	    let mut args = [
	        $( core::mem::transmute($args) ),*
	    ];
	    let args_ptr: *mut u64 = args.as_mut_ptr();
	    $crate::vprintf( concat!($fmt, "\0").as_ptr(), args_ptr as *mut _ )
	}
}
