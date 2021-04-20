extern "C" {
    #[ffi_const]
    #[link_name = "llvm.nvvm.bar.warp.sync"]
    fn __warp_sync(membermask: u32);
    #[ffi_const]
    #[link_name = "llvm.nvvm.barrier0"]
    fn __syncthreads();
    #[ffi_const]
    #[link_name = "llvm.nvvm.barrier0.and"]
    fn __syncthreads_and(test: u32) -> u32;
    #[ffi_const]
    #[link_name = "llvm.nvvm.barrier0.or"]
    fn __syncthreads_or(test: u32) -> u32;
    #[ffi_const]
    #[link_name = "llvm.nvvm.barrier0.popc"]
    fn __syncthreads_count(test: u32) -> u32;
}

#[doc = r#"
https://docs.nvidia.com/cuda/nvvm-ir-spec/index.html#nvvm-intrin-warp-level-sync

This intrinsic causes executing thread to wait until all threads corresponding to 
%membermask have executed the same intrinsic with the same %membermask value before 
resuming execution. 

The argument %membership is a 32bit mask, with each bit corresponding to a lane 
in the warp. 1 means the thread is in the subset. 

The behavior of this intrinsic is undefined if any thread participating in the 
intrinsic has exited or the executing thread is not in the %membermask. 

For compute_62 or below, all threads in %membermask must call the same @llvm.nvvm.bar.warp.sync()
in convergence, and only threads belonging to the %membermask can be active when the intrinsic 
is called. Otherwise, the behavior is undefined.
"#]
pub fn warp_sync(membermask: u32) {
    unsafe { __warp_sync(membermask) }
}

/// waits until all threads in the thread block have reached this point and all global and shared memory
/// accesses made by these threads prior to llvm.nvvm.barrier0() are visible to all threads in the block.
pub fn syncthreads() {
    unsafe {
        __syncthreads();
    }
}

/// is identical to llvm.nvvm.barrier0() with the additional feature that it evaluates predicate for
/// all threads of the block and returns non-zero if and only if predicate evaluates to non-zero for all of them.
pub fn syncthreads_and(test: bool) -> bool {
    unsafe { __syncthreads_and(if test { 1 } else { 0 }) != 0 }
}

/// is identical to llvm.nvvm.barrier0() with the additional feature that it evaluates predicate for all threads
/// of the block and returns non-zero if and only if predicate evaluates to non-zero for any of them.
pub fn syncthreads_or(test: bool) -> bool {
    unsafe { __syncthreads_or(if test { 1 } else { 0 }) != 0 }
}

/// is identical to llvm.nvvm.barrier0() with the additional feature that it evaluates predicate for all
/// threads of the block and returns the number of threads for which predicate evaluates to non-zero.
pub fn syncthreads_count(test: bool) -> u32 {
    unsafe { __syncthreads_count(if test { 1 } else { 0 }) as u32 }
}
