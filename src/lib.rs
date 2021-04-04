#![no_std]
#![feature(core_intrinsics, asm, global_asm, link_llvm_intrinsics)]
#![cfg_attr(feature = "alloc", feature(alloc_error_handler))]
#![cfg_attr(
    all(feature = "panic", feature = "alloc"),
    feature(panic_info_message, fmt_as_str)
)]
#![allow(non_camel_case_types)]

#[cfg(feature = "alloc")]
extern crate alloc;

// TODO: Documentation, active-mask, warp matrix ops, memory barriers, asynchronous copy,
// builtin-redux, nanosleep, cfg guards for ptx isa version & sm version,
// clock sreg?, math rounding modes?, cooperative groups?, unstable-allocator-api?

/*
https://docs.nvidia.com/cuda/parallel-thread-execution/index.htm
https://docs.nvidia.com/cuda/ptx-writers-guide-to-interoperability/index.html
*/

#[cfg(feature = "alloc")]
mod allocator;
mod barrier;
mod float;
#[cfg(feature = "panic")]
mod panic;
mod shuffle;
mod sreg;
mod syscall;
pub use crate::barrier::*;
pub use crate::float::*;
pub use crate::shuffle::*;
pub use crate::sreg::*;
pub use crate::syscall::*;
use core::{intrinsics::transmute, ptr::NonNull};
pub use nvptx_vprintf::printf;

pub const ALL_MEMBER_MASK: u32 = 0xffffffff;

extern "C" {
    #[link_name = "llvm.nvvm.atomic.load.add.f32.p0f32"]
    pub fn atomic_load_add_f32(address: *mut f32, val: f32) -> f32;
    #[link_name = "llvm.nvvm.atomic.load.add.f64.p0f64"]
    pub fn atomic_load_add_f64(address: *mut f64, val: f64) -> f64;

    #[link_name = "llvm.nvvm.atomic.load.inc.32.p0i32"]
    pub fn atomic_load_inc_32(address: *mut u32, val: u32) -> u32;
    #[link_name = "llvm.nvvm.atomic.load.dec.32.p0i32"]
    pub fn atomic_load_dec_32(address: *mut u32, val: u32) -> u32;

    #[link_name = "llvm.nvvm.vote.all.sync"]
    fn vote_all_sync(membermask: u32, pred: bool) -> bool;
    #[link_name = "llvm.nvvm.vote.any.sync"]
    fn vote_any_sync(membermask: u32, pred: bool) -> bool;
    #[link_name = "llvm.nvvm.vote.uni.sync"]
    fn vote_uni_sync(membermask: u32, pred: bool) -> bool;
    #[link_name = "llvm.nvvm.vote.ballot.sync"]
    fn vote_ballot_sync(membermask: u32, pred: bool) -> u32;

    #[link_name = "llvm.nvvm.match.any.sync.i32"]
    fn match_any_i32_sync(membermask: u32, value: u32) -> u32;
    #[link_name = "llvm.nvvm.match.any.sync.i64"]
    fn match_any_i64_sync(membermask: u32, value: u64) -> u32;

    #[link_name = "llvm.nvvm.match.all.sync.i32p"]
    fn match_all_i32_sync(membermask: u32, value: u32) -> (u32, bool);
    #[link_name = "llvm.nvvm.match.all.sync.i64p"]
    fn match_all_i64_sync(membermask: u32, value: u64) -> (u32, bool);
}

/// true if the source predicates is true for all thread in %membermask, false otherwise
pub fn vote_all(membermask: u32, pred: bool) -> bool {
    unsafe { vote_all_sync(membermask, pred) }
}

/// true if the source predicate is true for any thread in %membermask, false otherwise
pub fn vote_any(membermask: u32, pred: bool) -> bool {
    unsafe { vote_any_sync(membermask, pred) }
}

/// true if the source predicates are the same for all thread in %membermask, false otherwise
pub fn vote_eq(membermask: u32, pred: bool) -> bool {
    unsafe { vote_uni_sync(membermask, pred) }
}

/// warp mask ballot data, containing the predicate value from each thread in %membermask
pub fn vote_ballot(membermask: u32, pred: bool) -> u32 {
    unsafe { vote_ballot_sync(membermask, pred) }
}

pub trait Match: Sized {
    fn match_any(self, membermask: u32) -> u32;
    fn match_all(self, membermask: u32) -> (u32, bool);
}

impl Match for u32 {
    fn match_any(self, membermask: u32) -> u32 {
        unsafe { match_any_i32_sync(membermask, self) }
    }
    fn match_all(self, membermask: u32) -> (u32, bool) {
        unsafe { match_all_i32_sync(membermask, self) }
    }
}

impl Match for i32 {
    fn match_any(self, membermask: u32) -> u32 {
        unsafe { match_any_i32_sync(membermask, transmute(self)) }
    }
    fn match_all(self, membermask: u32) -> (u32, bool) {
        unsafe { match_all_i32_sync(membermask, transmute(self)) }
    }
}

impl Match for f32 {
    fn match_any(self, membermask: u32) -> u32 {
        unsafe { match_any_i32_sync(membermask, transmute(self)) }
    }
    fn match_all(self, membermask: u32) -> (u32, bool) {
        unsafe { match_all_i32_sync(membermask, transmute(self)) }
    }
}

impl Match for u64 {
    fn match_any(self, membermask: u32) -> u32 {
        unsafe { match_any_i64_sync(membermask, self) }
    }
    fn match_all(self, membermask: u32) -> (u32, bool) {
        unsafe { match_all_i64_sync(membermask, self) }
    }
}

impl Match for i64 {
    fn match_any(self, membermask: u32) -> u32 {
        unsafe { match_any_i64_sync(membermask, transmute(self)) }
    }
    fn match_all(self, membermask: u32) -> (u32, bool) {
        unsafe { match_all_i64_sync(membermask, transmute(self)) }
    }
}

impl Match for f64 {
    fn match_any(self, membermask: u32) -> u32 {
        unsafe { match_any_i64_sync(membermask, transmute(self)) }
    }
    fn match_all(self, membermask: u32) -> (u32, bool) {
        unsafe { match_all_i64_sync(membermask, transmute(self)) }
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

pub unsafe fn dynamic_shared_array<T: 'static + Sized + Send + Sync>(
    n: usize,
) -> Option<NonNull<[T]>> {
    let mut shared_ptr;
    let mut dyn_mem_size: u32;
    asm!(
        "cvta.shared.u64 {ptr}, dynamic_shared_memory; mov.u32 {sz}, %dynamic_smem_size;",
        ptr = out(reg64) shared_ptr,
        sz = out(reg32) dyn_mem_size,
        options(pure, nomem, nostack, preserves_flags)
    );
    if n * core::mem::size_of::<T>() <= dyn_mem_size as usize {
        Some(NonNull::new_unchecked(core::ptr::slice_from_raw_parts_mut(
            shared_ptr, n,
        )))
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
