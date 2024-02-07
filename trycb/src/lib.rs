#![deny(clippy::all)]

use std::thread;

use napi::{
  bindgen_prelude::*, threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode}, JsObject, Ref
};

#[macro_use]
extern crate napi_derive;

struct CallbackContext {
  callback: Ref<()>,
}

#[napi]
struct SimpleCall {
  cb: Option<JsObject>,
  env: Option<Env>,
  tsfn: Option<ThreadsafeFunction<u32, ErrorStrategy::CalleeHandled>>
}

#[napi]
impl SimpleCall {
  #[napi(constructor)]
  pub fn new() -> Self {
    SimpleCall {
      cb: None,
      env: None,
      tsfn: None,
    }
  }

  #[napi]
  pub fn register_cb(&mut self, env: Env, cb: JsFunction) -> Result<()> {
    let mut js_obj = env.create_object()?;
    let js_fn_ref = env.create_reference(cb).unwrap();
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
    if let Some(env) = &self.env {
      if let Some(js_obj) = &self.cb {
        let ctx: &mut CallbackContext = env.unwrap(&js_obj)?;
        let js_fn: JsFunction = env.get_reference_value(&ctx.callback)?;
        let _ = js_fn.call_without_args(None);   
      }
    }
    Ok(())
  }


  #[napi] 
  pub fn register_tsfn(&mut self, cb: JsFunction) -> Result<()> {
    let tsfn: ThreadsafeFunction<u32, ErrorStrategy::CalleeHandled> = cb
    .create_threadsafe_function(0, |ctx| {
      ctx.env.create_uint32(ctx.value + 1).map(|v| vec![v])
    })?;
    self.tsfn = Some(tsfn);
    Ok(())
  }

  #[napi]
  pub fn try_tsfn(&self) -> Result<()> {
    if let Some(tsfn) = &self.tsfn {
      let f = tsfn.clone();
      f.call(Ok(100), ThreadsafeFunctionCallMode::Blocking);
    }

    Ok(())
  }
}