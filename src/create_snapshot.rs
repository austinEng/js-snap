extern crate rusty_v8;
use rusty_v8 as v8;

fn main() {
  let source_file = std::env::args().nth(1).unwrap();
  let out_file = std::env::args().nth(2).unwrap();
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