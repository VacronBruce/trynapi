#![deny(clippy::all)]

use napi::{ threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode}, Env, JsFunction, JsObject, Ref, Result};

#[macro_use]
extern crate napi_derive;

struct CallbackContext {
  callback: Ref<()>,
}

#[napi]
struct SimpleCall {
  cb: Option<JsObject>,
  env: Option<Env>,
}

#[napi]
impl SimpleCall {
  #[napi(constructor)]
  pub fn new() -> Self {
    SimpleCall {
      cb: None,
      env: None,
    }
  }

  #[napi]
  pub fn register_cb(&mut self, env: Env, cb: JsFunction) -> Result<()> {
    let mut js_obj = env.create_object()?;
    let js_fn_ref = env.create_reference_with_refcount(cb, 10).unwrap();
    
    let ctx = CallbackContext {
      callback: js_fn_ref,
    };
    
    env.wrap(&mut js_obj, ctx)?;
    
    self.cb = Some(js_obj);
    self.env = Some(env);
    Ok(())
  }

  #[napi]
  pub fn try_cb(&mut self) -> Result<()> {
    if let Some(env) = &mut self.env {
      if let Some(js_obj) = &self.cb {
        let ctx: &mut CallbackContext = env.unwrap(&js_obj)?;
        let js_fn: JsFunction = env.get_reference_value(&ctx.callback)?;
        let _ = js_fn.call_without_args(None);
      }
    }
    Ok(())
  }
}
