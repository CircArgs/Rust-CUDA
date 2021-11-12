#![feature(asm)]
#![cfg_attr(
    target_os = "cuda",
    no_std,
    feature(register_attr),
    register_attr(nvvm_internal)
)]
// #![deny(warnings)]

use core::mem::MaybeUninit;

use cuda_std::*;
use cust_core::DeviceCopy;

use optix_device as optix;

extern crate alloc;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct LaunchParams {
    pub frame_id: i32,
    pub fb_size: [u32; 2],
    pub color_buffer: u64,
}

unsafe impl DeviceCopy for LaunchParams {}

#[no_mangle]
static PARAMS: MaybeUninit<LaunchParams> = MaybeUninit::uninit();

extern "C" {
    pub fn vprintf(format: *const u8, valist: *const core::ffi::c_void) -> i32;
}

#[kernel]
pub unsafe fn __closesthit__radiance() {}

#[kernel]
pub unsafe fn __anyhit__radiance() {}

#[kernel]
pub unsafe fn __miss__radiance() {}

#[kernel]
pub unsafe fn __raygen__renderFrame() {
    // let ix = _optix_get_launch_index_x();
    // let iy = _optix_get_launch_index_y();

    let ix: u32;
    let iy: u32;

    let idx = optix::get_launch_index();

    let params = PARAMS.assume_init_ref();

    if idx[0] == 3 && idx[1] == 4 {
        vprintf(
            b"Hello from Rust kernel!\n\0".as_ptr().cast(),
            0 as *const core::ffi::c_void,
        );

        vprintf(
            b"frame id is %d\n\0".as_ptr().cast(),
            &params.frame_id as *const _ as *const core::ffi::c_void,
        );
    }
}

// #[kernel]
// pub unsafe fn render(fb: *mut Vec3<f32>, view: &Viewport) {}
