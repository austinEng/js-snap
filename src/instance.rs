use rusty_v8 as v8;

use std::collections::HashMap;

pub struct Instance<'p> {
  isolate: *mut v8::OwnedIsolate,
  handle_scope: *mut v8::Scope<'p, v8::HandleScope, v8::OwnedIsolate>,
  scope: &'p mut v8::scope::Entered<'p, v8::HandleScope, v8::OwnedIsolate>,
  fn_map: HashMap<String, v8::Local::<'p, v8::Function>>,
  reused_output: String,
}

impl Drop for Instance<'_> {
  fn drop(&mut self) {
    self.fn_map.clear();
    unsafe { Box::from_raw(self.handle_scope) };
    unsafe { Box::from_raw(self.isolate) };
  }
}

pub trait Allocated<T: ?Sized>:
  std::ops::Deref<Target = T> + std::borrow::Borrow<T> + 'static
{
}
impl<A, T: ?Sized> Allocated<T> for A where
  A: std::ops::Deref<Target = T> + std::borrow::Borrow<T> + 'static
{
}

impl<'p> Instance<'p> {
  fn from_isolate_with_closure<F>(isolate: v8::OwnedIsolate, f: F) -> Self
    where F: FnOnce(
      // v8::Local::<'i, v8::Context>,
      &mut v8::scope::Entered<'p, v8::HandleScope, v8::OwnedIsolate>,
      &mut HashMap<String, v8::Local::<'p, v8::Function>>,
    ) -> ()
  {

    let isolate = Box::into_raw(Box::new(isolate));
  
    let handle_scope = unsafe {
      Box::into_raw(Box::new(v8::HandleScope::new(&mut *isolate)))
    };

    let scope = unsafe {
      (*handle_scope).enter()
    };
    
    let mut fn_map = HashMap::new();
    f(scope, &mut fn_map);

    Instance {
      isolate,
      handle_scope,
      scope,
      fn_map,
      reused_output: String::new(),
    }
  }

  fn extract_exports<'s, 'p2: 's, I>(
      context: v8::Local::<'p2, v8::Context>,
      scope: &mut v8::scope::Entered<'s, v8::ContextScope, v8::scope::Entered<'p2, v8::HandleScope, I>>,
      exports: v8::Local<'s, v8::Object>,
      fn_map: &mut HashMap<String, v8::Local::<'p2, v8::Function>>)
    where I: v8::InIsolate
  {
    if let Some(props) = exports.get_own_property_names(scope, context) {
      for i in 0..props.length() {
        let name = props.get_index(scope, context, i).unwrap();
        
        let mut escape_scope = v8::EscapableHandleScope::new(scope);
        let scope = escape_scope.enter();

        assert!(name.is_name());
        let func = exports.get(scope, context, name).unwrap();
        
        let func = unsafe {
          v8::Local::<v8::Function>::cast(func)
        };

        fn_map.insert(
          name.to_string(scope).unwrap().to_rust_string_lossy(scope),
          scope.escape(func)
        );
      }
    }
  }

  pub fn from_source(js_code: &str, export_name: &str) -> Self {    
    Self::from_isolate_with_closure(v8::Isolate::new(Default::default()),
                                    |scope, fn_map| {
      let context = v8::Context::new(scope);
      let mut context_scope = v8::ContextScope::new(scope, context);
      let scope = context_scope.enter();

      let code = v8::String::new(scope, &[js_code, export_name].concat()).unwrap();

      let mut script = v8::Script::compile(scope, context, code, None).unwrap();
      let exports: v8::Local<v8::Value> = script.run(scope, context).unwrap();
      let exports = exports.to_object(scope).unwrap();

      Self::extract_exports(context, scope, exports, fn_map);
    })
  }

  pub fn from_snapshot(data: impl Allocated<[u8]>, export_name: &str) -> Self {
    let isolate_params: v8::CreateParams = Default::default();
    let isolate_params = isolate_params.snapshot_blob(data);

    Self::from_isolate_with_closure(v8::Isolate::new(isolate_params),
                                    |scope, fn_map| {
      let context = v8::Context::new(scope);
      let mut context_scope = v8::ContextScope::new(scope, context);
      let scope = context_scope.enter();

      let code = v8::String::new(scope, &[export_name].concat()).unwrap();
      let mut code = v8::Script::compile(scope, context, code, None).unwrap();
      let exports: v8::Local<v8::Value> = code.run(scope, context).unwrap();
      let exports = exports.to_object(scope).unwrap();

      Self::extract_exports(context, scope, exports, fn_map);
    })
  }

  pub fn call(&mut self, name: &str, params: &str) -> Option<std::string::String> {
    let mut handle_scope = v8::HandleScope::new(self.scope);
    let scope = handle_scope.enter();

    let context = v8::Context::new(scope);
    let mut context_scope = v8::ContextScope::new(scope, context);
    let scope = context_scope.enter();

    let params: v8::Local<v8::Value> = match v8::String::new(scope, params) {
      Some(s) => s.into(),
      None =>  v8::undefined(scope).into()
    };

    let undef = v8::undefined(scope).into();
    let result = self.fn_map[name].call(
      scope, context, undef, &[params])?;
    let result = result.to_string(scope)?;
    
    Some(result.to_rust_string_lossy(scope))
  }

  pub fn call_for_ffi(&mut self, name: &str, params: &str) -> Option<&[u8]> {
    self.reused_output = self.call(name, params)?;
    Some(self.reused_output.as_bytes())
  }
}