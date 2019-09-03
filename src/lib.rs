use std::ffi::{CStr, CString};
use std::os::raw::c_char;

extern crate libc;
pub use uni_view::*;

mod geometry;
pub mod math;
mod texture;

mod utils;
pub use utils::{depth_stencil, matrix_helper};

mod shader;
mod vertex;

mod sdf;
pub use sdf::SDFTextView;

use math::Position;

pub trait SurfaceView {
    fn resize(&mut self);
    fn scale(&mut self, scale: f32);
    fn touch_moved(&mut self, position: Position);

    fn enter_frame(&mut self);
}

#[cfg(not(target_os = "macos"))]
#[no_mangle]
pub extern "C" fn create_sdf_view(view: uni_view::AppViewObj) -> *mut libc::c_void {
    let rust_view = uni_view::AppView::new(view);
    let obj = SDFTextView::new(rust_view);
    box_obj(obj)
}

#[cfg(not(target_os = "macos"))]
#[no_mangle]
pub unsafe extern "C" fn sdf_view_set_bundle_image(obj: *mut libc::c_void, image_name: *mut c_char) {
    let c_str = CStr::from_ptr(image_name);
    let name = match c_str.to_str() {
        Err(_) => "",
        Ok(string) => string,
    };

    let mut obj: Box<Box<SDFTextView>> = Box::from_raw(obj as *mut _);
    obj.bundle_image(name.to_string());
    let _ = Box::into_raw(obj) as *mut libc::c_void;
}

#[cfg(not(target_os = "macos"))]
fn box_obj(obj: impl SurfaceView) -> *mut libc::c_void {
    let boxed_trait: Box<dyn SurfaceView> = Box::new(obj);
    let boxed_boxed_trait = Box::new(boxed_trait);
    let heap_pointer = Box::into_raw(boxed_boxed_trait);
    // let boxed_boxed_trait = Box::new(v);
    // let heap_pointer = Box::into_raw(boxed_boxed_trait);
    heap_pointer as *mut libc::c_void
}

#[cfg(not(target_os = "macos"))]
#[no_mangle]
pub unsafe extern "C" fn enter_frame(obj: *mut libc::c_void) -> *mut libc::c_void {
    let mut obj: Box<Box<dyn SurfaceView>> = Box::from_raw(obj as *mut _);
    obj.enter_frame();

    // 重新将所有权移出
    Box::into_raw(obj) as *mut libc::c_void
}

#[cfg(not(target_os = "macos"))]
#[no_mangle]
pub unsafe extern "C" fn touch_move(obj: *mut libc::c_void, p: TouchPoint) {
    let mut obj: Box<Box<dyn SurfaceView>> = Box::from_raw(obj as *mut _);
    obj.touch_moved(crate::math::Position::new(p.x, p.y));

    // 重新将所有权移出
    let _ = Box::into_raw(obj) as *mut libc::c_void;
}

#[cfg(not(target_os = "macos"))]
#[no_mangle]
pub unsafe extern "C" fn resize(obj: *mut libc::c_void, _p: TouchPoint) {
    let mut obj: Box<Box<dyn SurfaceView>> = Box::from_raw(obj as *mut _);
    obj.resize();

    let _ = Box::into_raw(obj) as *mut libc::c_void;
}

#[cfg(not(target_os = "macos"))]
#[no_mangle]
pub unsafe extern "C" fn scale(obj: *mut libc::c_void, scale: f32) {
    let mut obj: Box<Box<dyn SurfaceView>> = Box::from_raw(obj as *mut _);
    obj.scale(scale);

    let _ = Box::into_raw(obj) as *mut libc::c_void;
}
