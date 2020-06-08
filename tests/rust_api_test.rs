use js_snap::*;

#[test]
fn rust_api_simple() {
  JSSnap::init();
  let mut instance = Instance::from_source("const fns = { Greet: (params) => params };", "fns");
  let result = instance.call("Greet", "Hello Rust").unwrap();
  println!("\nresult:\n{}", result);
  assert_eq!(result, "Hello Rust");
}