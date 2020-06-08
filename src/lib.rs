#[macro_use]
extern crate lazy_static;

extern crate rusty_v8;

use rusty_v8 as v8;

mod instance;

pub use instance::Instance;

pub struct JSSnap {
}

impl JSSnap {
  pub fn init() {
    lazy_static! {
      static ref INIT: () = (|| {
        let platform = v8::new_default_platform().unwrap();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();
      })();
    }
    lazy_static::initialize(&INIT);
  }
}

#[no_mangle]
pub extern fn js_snap_init() -> () {
  JSSnap::init();
}

#[no_mangle]
pub extern fn js_snap_instance_from_source<'a>(
    source: *const std::os::raw::c_char,
    export_name: *const std::os::raw::c_char) -> *mut Instance<'a>
{
  let source = unsafe { std::ffi::CStr::from_ptr(source) };
  let source = source.to_str().unwrap();
  let export_name = unsafe { std::ffi::CStr::from_ptr(export_name) };
  let export_name = export_name.to_str().unwrap();
  Box::into_raw(Box::new(Instance::from_source(source, export_name)))
}

#[no_mangle]
pub extern fn js_snap_instance_from_snapshot<'a>(
    data: *const u8,
    data_length: usize,
    export_name: *const std::os::raw::c_char) -> *mut Instance<'a>
{
  let blob = unsafe { std::slice::from_raw_parts(data, data_length) };
  let export_name = unsafe { std::ffi::CStr::from_ptr(export_name) };
  let export_name = export_name.to_str().unwrap();
  Box::into_raw(Box::new(Instance::from_snapshot(blob, export_name)))
}

#[no_mangle]
pub extern fn js_snap_instance_delete<'a>(instance: *mut Instance<'a>) -> () {
  unsafe { Box::from_raw(instance) };
}

#[no_mangle]
pub extern fn js_snap_instance_call<'a>(
  instance: *mut Instance<'a>,
  name: *const std::os::raw::c_char,
  params: *const std::os::raw::c_char,
  result_ptr: *mut *const std::os::raw::c_char,
  result_len: *mut i32,
) -> () {
  let name = unsafe { std::ffi::CStr::from_ptr(name) };
  let name = name.to_str().unwrap();

  let params = if params == std::ptr::null() {
    "{}"
  } else {
    let params = unsafe { std::ffi::CStr::from_ptr(params) };
    params.to_str().unwrap()
  };

  let instance = unsafe { &mut *instance };

  use std::convert::TryFrom;

  match instance.call_for_ffi(name, params) {
    Some(result) => {
      unsafe { *result_ptr = result.as_ptr() as *const std::os::raw::c_char };
      unsafe { *result_len = match i32::try_from(result.len()) {
        Ok(len) => len, 
        Err(_) => 0
      }};
    }
    None => {
      unsafe { *result_ptr = std::ptr::null() };
      unsafe { *result_len = 0 };
    }
  }
}
