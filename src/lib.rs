#![no_std]
#![feature(core_intrinsics, link_llvm_intrinsics, ffi_const)]
#![cfg_attr(feature = "panic", feature(panic_info_message))]
#![cfg_attr(feature = "unstable-allocator-api", feature(allocator_api))]
#![allow(non_camel_case_types)]
#![deny(unsafe_op_in_unsafe_fn)]

#[cfg(feature = "global-allocator")]
extern crate alloc;
#[macro_use]
extern crate derive_more;

// TODO: Documentation, active-mask, warp matrix ops, memory barriers, asynchronous copy,
// builtin-redux, nanosleep, cfg guards for ptx isa version & sm version,
// clock sreg?, math rounding modes?, cooperative groups?, unstable-allocator-api?

/*
https://docs.nvidia.com/cuda/parallel-thread-execution/index.htm
https://docs.nvidia.com/cuda/ptx-writers-guide-to-interoperability/index.html
*/

mod allocator;
mod barrier;
mod float;
#[cfg(feature = "panic")]
mod panic;
mod shuffle;
mod sreg;
mod syscall;
pub use crate::allocator::CudaSysAllocator;
pub use crate::barrier::*;
pub use crate::float::*;
pub use crate::shuffle::*;
pub use crate::sreg::*;
pub use crate::syscall::*;
use core::mem::transmute;
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

    #[ffi_const]
    #[link_name = "llvm.nvvm.vote.all.sync"]
    fn vote_all_sync(membermask: u32, pred: bool) -> bool;
    #[ffi_const]
    #[link_name = "llvm.nvvm.vote.any.sync"]
    fn vote_any_sync(membermask: u32, pred: bool) -> bool;
    #[ffi_const]
    #[link_name = "llvm.nvvm.vote.uni.sync"]
    fn vote_uni_sync(membermask: u32, pred: bool) -> bool;
    #[ffi_const]
    #[link_name = "llvm.nvvm.vote.ballot.sync"]
    fn vote_ballot_sync(membermask: u32, pred: bool) -> u32;

    #[ffi_const]
    #[link_name = "llvm.nvvm.match.any.sync.i32"]
    fn match_any_i32_sync(membermask: u32, value: u32) -> u32;
    #[ffi_const]
    #[link_name = "llvm.nvvm.match.any.sync.i64"]
    fn match_any_i64_sync(membermask: u32, value: u64) -> u32;

    // #[link_name = "llvm.nvvm.match.all.sync.i32p"]
    // fn match_all_i32_sync(membermask: u32, value: u32) -> (u32, bool);
    // #[link_name = "llvm.nvvm.match.all.sync.i64p"]
    // fn match_all_i64_sync(membermask: u32, value: u64) -> (u32, bool);
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
    // fn match_all(self, membermask: u32) -> (u32, bool);
}

impl Match for u32 {
    fn match_any(self, membermask: u32) -> u32 {
        unsafe { match_any_i32_sync(membermask, self) }
    }
    // fn match_all(self, membermask: u32) -> (u32, bool) {
    //     unsafe { match_all_i32_sync(membermask, self) }
    // }
}

impl Match for i32 {
    fn match_any(self, membermask: u32) -> u32 {
        unsafe { match_any_i32_sync(membermask, transmute(self)) }
    }
    // fn match_all(self, membermask: u32) -> (u32, bool) {
    //     unsafe { match_all_i32_sync(membermask, transmute(self)) }
    // }
}

impl Match for f32 {
    fn match_any(self, membermask: u32) -> u32 {
        unsafe { match_any_i32_sync(membermask, self.to_bits()) }
    }
    // fn match_all(self, membermask: u32) -> (u32, bool) {
    //     unsafe { match_all_i32_sync(membermask, self.to_bits()) }
    // }
}

impl Match for u64 {
    fn match_any(self, membermask: u32) -> u32 {
        unsafe { match_any_i64_sync(membermask, self) }
    }
    // fn match_all(self, membermask: u32) -> (u32, bool) {
    //     unsafe { match_all_i64_sync(membermask, self) }
    // }
}

impl Match for i64 {
    fn match_any(self, membermask: u32) -> u32 {
        unsafe { match_any_i64_sync(membermask, transmute(self)) }
    }
    // fn match_all(self, membermask: u32) -> (u32, bool) {
    //     unsafe { match_all_i64_sync(membermask, transmute(self)) }
    // }
}

impl Match for f64 {
    fn match_any(self, membermask: u32) -> u32 {
        unsafe { match_any_i64_sync(membermask, self.to_bits()) }
    }
    // fn match_all(self, membermask: u32) -> (u32, bool) {
    //     unsafe { match_all_i64_sync(membermask, self.to_bits()) }
    // }
}

// #[repr(transparent)]
// pub struct AtomicF32 {
//     inner: UnsafeCell<f32>
// }
//
// impl AtomicF32 {
//
// }
