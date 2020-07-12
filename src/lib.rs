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

#[cfg(feature = "snapshot_bundle")]
#[no_mangle]
pub extern fn js_snap_instance_from_bundle<'a>(
    export_name: *const std::os::raw::c_char) -> *mut Instance<'a>
{
  let bytes: &'static [u8] = include_bytes!(env!("JS_SNAPSHOT_BUNDLE"));
  js_snap_instance_from_snapshot(
    bytes.as_ptr(),
    bytes.len(),
    export_name)
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

pub fn create_snapshot( source_file: String, out_file: String) {
  let code = std::fs::read_to_string(source_file).unwrap();

  let platform = v8::new_default_platform().unwrap();
  v8::V8::initialize_platform(platform);
  v8::V8::initialize();

  let mut creator = v8::SnapshotCreator::new(Option::None);
  let mut isolate = unsafe { creator.get_owned_isolate() };
  {
    let mut handle_scope = v8::HandleScope::new(&mut isolate);
    let scope = handle_scope.enter();

    let context = v8::Context::new(scope);
    creator.set_default_context(context);

    let mut context_scope = v8::ContextScope::new(scope, context);
    let scope = context_scope.enter();

    let code = v8::String::new(scope, &code).unwrap();
    let mut code = v8::Script::compile(scope, context, code, None).unwrap();
    code.run(scope, context);
  }
  std::mem::forget(isolate);

  let blob = creator.create_blob(
    v8::FunctionCodeHandling::Clear).unwrap();

  std::fs::write(out_file, blob).unwrap();
}
