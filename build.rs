extern crate cbindgen;

fn main() {
  let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
  cbindgen::Builder::new()
    .with_crate(crate_dir)
    .with_language(cbindgen::Language::C)
    .with_include_guard("JS_SNAP_H_")
    .with_item_prefix("JSSnap")
    .generate()
    .expect("Unable to generate C bindings")
    .write_to_file("src/include/js_snap.h");
}