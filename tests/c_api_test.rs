use js_snap::*;

#[test]
fn c_api_simple() {
  js_snap_init();
  let instance = js_snap_instance_from_source(
    std::ffi::CString::new("const fns = { Greet: (params) => params };").unwrap().as_ptr(),
    std::ffi::CString::new("fns").unwrap().as_ptr());

  let mut result_ptr: *const std::os::raw::c_char = std::ptr::null();
  let mut result_len: i32 = 0;

  let result_ptr_ptr = &mut result_ptr as *mut *const std::os::raw::c_char;
  let result_len_ptr = &mut result_len as *mut i32;

  js_snap_instance_call(
    instance,
    std::ffi::CString::new("Greet").unwrap().as_ptr(),
    std::ffi::CString::new("Hello C").unwrap().as_ptr(),
    result_ptr_ptr,
    result_len_ptr);

  assert_ne!(result_len, 0);
  let result = unsafe {
    std::slice::from_raw_parts(result_ptr as *const u8, result_len as usize)
  };

  let result = std::str::from_utf8(result).unwrap();
  println!("\nc_result:\n{}", result);
  assert_eq!(result, "Hello C");

  js_snap_instance_delete(instance);
}
