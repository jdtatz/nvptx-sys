extern "C" {
    #[link_name = "llvm.nvvm.read.ptx.sreg.tid.x"]
    fn read_ptx_sreg_tid_x() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.tid.y"]
    fn read_ptx_sreg_tid_y() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.tid.z"]
    fn read_ptx_sreg_tid_z() -> u32;

    #[link_name = "llvm.nvvm.read.ptx.sreg.ntid.x"]
    fn read_ptx_sreg_ntid_x() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.ntid.y"]
    fn read_ptx_sreg_ntid_y() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.ntid.z"]
    fn read_ptx_sreg_ntid_z() -> u32;

    #[link_name = "llvm.nvvm.read.ptx.sreg.ctaid.x"]
    fn read_ptx_sreg_ctaid_x() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.ctaid.y"]
    fn read_ptx_sreg_ctaid_y() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.ctaid.z"]
    fn read_ptx_sreg_ctaid_z() -> u32;

    #[link_name = "llvm.nvvm.read.ptx.sreg.nctaid.x"]
    fn read_ptx_sreg_nctaid_x() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.nctaid.y"]
    fn read_ptx_sreg_nctaid_y() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.nctaid.z"]
    fn read_ptx_sreg_nctaid_z() -> u32;

    #[link_name = "llvm.nvvm.read.ptx.sreg.warpsize"]
    fn read_nvvm_read_ptx_sreg_warpsize() -> u32;

    #[link_name = "llvm.nvvm.read.ptx.sreg.laneid"]
    fn read_nvvm_read_ptx_sreg_laneid() -> u32;
}

pub struct threadIdx {}
pub struct blockIdx {}
pub struct blockDim {}
pub struct gridDim {}

impl threadIdx {
    pub fn x() -> u32 {
        unsafe { read_ptx_sreg_tid_x() }
    }
    pub fn y() -> u32 {
        unsafe { read_ptx_sreg_tid_y() }
    }
    pub fn z() -> u32 {
        unsafe { read_ptx_sreg_tid_z() }
    }
}

impl blockIdx {
    pub fn x() -> u32 {
        unsafe { read_ptx_sreg_ctaid_x() }
    }
    pub fn y() -> u32 {
        unsafe { read_ptx_sreg_ctaid_y() }
    }
    pub fn z() -> u32 {
        unsafe { read_ptx_sreg_ctaid_z() }
    }
}

impl blockDim {
    pub fn x() -> u32 {
        unsafe { read_ptx_sreg_ntid_x() }
    }
    pub fn y() -> u32 {
        unsafe { read_ptx_sreg_ntid_y() }
    }
    pub fn z() -> u32 {
        unsafe { read_ptx_sreg_ntid_z() }
    }
}

impl gridDim {
    pub fn x() -> u32 {
        unsafe { read_ptx_sreg_nctaid_x() }
    }
    pub fn y() -> u32 {
        unsafe { read_ptx_sreg_nctaid_y() }
    }
    pub fn z() -> u32 {
        unsafe { read_ptx_sreg_nctaid_z() }
    }
}

pub fn laneid() -> u32 {
    unsafe { read_nvvm_read_ptx_sreg_laneid() }
}

pub fn warpsize() -> u32 {
    unsafe { read_nvvm_read_ptx_sreg_warpsize() }
}
