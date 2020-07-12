extern crate js_snap;

fn main() {
  let source_file = std::env::args().nth(1).unwrap();
  let out_file = std::env::args().nth(2).unwrap();
  js_snap::create_snapshot(source_file, out_file);
}