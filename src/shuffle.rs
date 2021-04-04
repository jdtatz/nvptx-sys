extern "C" {
    #[link_name = "llvm.nvvm.shfl.sync.idx.i32p"]
    pub fn shfl_idx_sync_i32p(
        membermask: u32,
        val: u32,
        src_lane: u32,
        packing: u32,
    ) -> (u32, bool);
    #[link_name = "llvm.nvvm.shfl.sync.down.i32p"]
    pub fn shfl_down_sync_i32p(membermask: u32, val: u32, delta: u32, packing: u32) -> (u32, bool);
    #[link_name = "llvm.nvvm.shfl.sync.up.i32p"]
    pub fn shfl_up_sync_i32p(membermask: u32, val: u32, delta: u32, packing: u32) -> (u32, bool);
    #[link_name = "llvm.nvvm.shfl.sync.bfly.i32p"]
    pub fn shfl_bfly_sync_i32p(
        membermask: u32,
        val: u32,
        lane_mask: u32,
        packing: u32,
    ) -> (u32, bool);

    #[link_name = "llvm.nvvm.shfl.sync.idx.f32p"]
    fn shfl_idx_sync_f32p(membermask: u32, val: f32, src_lane: u32, packing: u32) -> (f32, bool);
    #[link_name = "llvm.nvvm.shfl.sync.down.f32p"]
    fn shfl_down_sync_f32p(membermask: u32, val: f32, delta: u32, packing: u32) -> (f32, bool);
    #[link_name = "llvm.nvvm.shfl.sync.up.f32p"]
    fn shfl_up_sync_f32p(membermask: u32, val: f32, delta: u32, packing: u32) -> (f32, bool);
    #[link_name = "llvm.nvvm.shfl.sync.bfly.f32p"]
    fn shfl_bfly_sync_f32p(membermask: u32, val: f32, lane_mask: u32, packing: u32) -> (f32, bool);
}

pub trait Shuffle: Sized {
    fn shfl_idx(self, mask: u32, src_lane: u32) -> Self {
        self.shfl_idx_p(mask, src_lane).0
    }
    fn shfl_down(self, mask: u32, delta: u32) -> Self {
        self.shfl_down_p(mask, delta).0
    }
    fn shfl_up(self, mask: u32, delta: u32) -> Self {
        self.shfl_up_p(mask, delta).0
    }
    fn shfl_bfly(self, mask: u32, lane_mask: u32) -> Self {
        self.shfl_bfly_p(mask, lane_mask).0
    }
    fn shfl_idx_p(self, mask: u32, src_lane: u32) -> (Self, bool);
    fn shfl_down_p(self, mask: u32, delta: u32) -> (Self, bool);
    fn shfl_up_p(self, mask: u32, delta: u32) -> (Self, bool);
    fn shfl_bfly_p(self, mask: u32, lane_mask: u32) -> (Self, bool);
}

impl Shuffle for u32 {
    fn shfl_idx_p(self, mask: u32, src_lane: u32) -> (Self, bool) {
        unsafe { shfl_idx_sync_i32p(mask, self, src_lane, 0x1f) }
    }

    fn shfl_down_p(self, mask: u32, delta: u32) -> (Self, bool) {
        unsafe { shfl_down_sync_i32p(mask, self, delta, 0x1f) }
    }

    fn shfl_up_p(self, mask: u32, delta: u32) -> (Self, bool) {
        unsafe { shfl_up_sync_i32p(mask, self, delta, 0) }
    }

    fn shfl_bfly_p(self, mask: u32, lane_mask: u32) -> (Self, bool) {
        unsafe { shfl_bfly_sync_i32p(mask, self, lane_mask, 0x1f) }
    }
}

impl Shuffle for i32 {
    fn shfl_idx_p(self, mask: u32, src_lane: u32) -> (Self, bool) {
        unsafe {
            let v = core::mem::transmute::<i32, u32>(self);
            let (r, p) = shfl_idx_sync_i32p(mask, v, src_lane, 0x1f);
            (core::mem::transmute::<u32, i32>(r), p)
        }
    }

    fn shfl_down_p(self, mask: u32, delta: u32) -> (Self, bool) {
        unsafe {
            let v = core::mem::transmute::<i32, u32>(self);
            let (r, p) = shfl_down_sync_i32p(mask, v, delta, 0x1f);
            (core::mem::transmute::<u32, i32>(r), p)
        }
    }

    fn shfl_up_p(self, mask: u32, delta: u32) -> (Self, bool) {
        unsafe {
            let v = core::mem::transmute::<i32, u32>(self);
            let (r, p) = shfl_up_sync_i32p(mask, v, delta, 0x1f);
            (core::mem::transmute::<u32, i32>(r), p)
        }
    }

    fn shfl_bfly_p(self, mask: u32, lane_mask: u32) -> (Self, bool) {
        unsafe {
            let v = core::mem::transmute::<i32, u32>(self);
            let (r, p) = shfl_bfly_sync_i32p(mask, v, lane_mask, 0x1f);
            (core::mem::transmute::<u32, i32>(r), p)
        }
    }
}

impl Shuffle for f32 {
    fn shfl_idx_p(self, mask: u32, src_lane: u32) -> (Self, bool) {
        unsafe { shfl_idx_sync_f32p(mask, self, src_lane, 0x1f) }
    }

    fn shfl_down_p(self, mask: u32, delta: u32) -> (Self, bool) {
        unsafe { shfl_down_sync_f32p(mask, self, delta, 0x1f) }
    }

    fn shfl_up_p(self, mask: u32, delta: u32) -> (Self, bool) {
        unsafe { shfl_up_sync_f32p(mask, self, delta, 0) }
    }

    fn shfl_bfly_p(self, mask: u32, lane_mask: u32) -> (Self, bool) {
        unsafe { shfl_bfly_sync_f32p(mask, self, lane_mask, 0x1f) }
    }
}
