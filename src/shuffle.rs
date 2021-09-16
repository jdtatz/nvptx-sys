use crate::FastFloat;

extern "C" {
    #[ffi_const]
    #[link_name = "llvm.nvvm.shfl.sync.idx.i32"]
    pub fn shfl_idx_sync_i32(membermask: u32, val: u32, src_lane: u32, packing: u32) -> u32;
    #[ffi_const]
    #[link_name = "llvm.nvvm.shfl.sync.down.i32"]
    pub fn shfl_down_sync_i32(membermask: u32, val: u32, delta: u32, packing: u32) -> u32;
    #[ffi_const]
    #[link_name = "llvm.nvvm.shfl.sync.up.i32"]
    pub fn shfl_up_sync_i32(membermask: u32, val: u32, delta: u32, packing: u32) -> u32;
    #[ffi_const]
    #[link_name = "llvm.nvvm.shfl.sync.bfly.i32"]
    pub fn shfl_bfly_sync_i32(membermask: u32, val: u32, lane_mask: u32, packing: u32) -> u32;

    #[ffi_const]
    #[link_name = "llvm.nvvm.shfl.sync.idx.f32"]
    fn shfl_idx_sync_f32(membermask: u32, val: f32, src_lane: u32, packing: u32) -> f32;
    #[ffi_const]
    #[link_name = "llvm.nvvm.shfl.sync.down.f32"]
    fn shfl_down_sync_f32(membermask: u32, val: f32, delta: u32, packing: u32) -> f32;
    #[ffi_const]
    #[link_name = "llvm.nvvm.shfl.sync.up.f32"]
    fn shfl_up_sync_f32(membermask: u32, val: f32, delta: u32, packing: u32) -> f32;
    #[ffi_const]
    #[link_name = "llvm.nvvm.shfl.sync.bfly.f32"]
    fn shfl_bfly_sync_f32(membermask: u32, val: f32, lane_mask: u32, packing: u32) -> f32;
}

pub trait Shuffle: Sized {
    fn shfl_idx(self, mask: u32, src_lane: u32) -> Self;
    fn shfl_down(self, mask: u32, delta: u32) -> Self;
    fn shfl_up(self, mask: u32, delta: u32) -> Self;
    fn shfl_bfly(self, mask: u32, lane_mask: u32) -> Self;
}

impl Shuffle for u32 {
    fn shfl_idx(self, mask: u32, src_lane: u32) -> Self {
        unsafe { shfl_idx_sync_i32(mask, self, src_lane, 0x1f) }
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

impl Shuffle for i32 {
    fn shfl_idx(self, mask: u32, src_lane: u32) -> Self {
        unsafe {
            core::mem::transmute::<u32, i32>(shfl_idx_sync_i32(
                mask,
                core::mem::transmute::<i32, u32>(self),
                src_lane,
                0x1f,
            ))
        }
    }

    fn shfl_down(self, mask: u32, delta: u32) -> Self {
        unsafe {
            core::mem::transmute::<u32, i32>(shfl_idx_sync_i32(
                mask,
                core::mem::transmute::<i32, u32>(self),
                delta,
                0x1f,
            ))
        }
    }

    fn shfl_up(self, mask: u32, delta: u32) -> Self {
        unsafe {
            core::mem::transmute::<u32, i32>(shfl_idx_sync_i32(
                mask,
                core::mem::transmute::<i32, u32>(self),
                delta,
                0,
            ))
        }
    }

    fn shfl_bfly(self, mask: u32, lane_mask: u32) -> Self {
        unsafe {
            core::mem::transmute::<u32, i32>(shfl_idx_sync_i32(
                mask,
                core::mem::transmute::<i32, u32>(self),
                lane_mask,
                0x1f,
            ))
        }
    }
}

impl Shuffle for f32 {
    fn shfl_idx(self, mask: u32, src_lane: u32) -> Self {
        unsafe { shfl_idx_sync_f32(mask, self, src_lane, 0x1f) }
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

impl<F: Shuffle> Shuffle for FastFloat<F> {
    fn shfl_idx(self, mask: u32, src_lane: u32) -> Self {
        FastFloat(self.0.shfl_idx(mask, src_lane))
    }

    fn shfl_down(self, mask: u32, delta: u32) -> Self {
        FastFloat(self.0.shfl_down(mask, delta))
    }

    fn shfl_up(self, mask: u32, delta: u32) -> Self {
        FastFloat(self.0.shfl_up(mask, delta))
    }

    fn shfl_bfly(self, mask: u32, lane_mask: u32) -> Self {
        FastFloat(self.0.shfl_bfly(mask, lane_mask))
    }
}
