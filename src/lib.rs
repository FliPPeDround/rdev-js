#![deny(clippy::all)]

use std::thread;

use napi::{
  bindgen_prelude::Uint32Array, JsFunction, Result,
  threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode},
};
use rdev::{
  display_size, listen, simulate, Button, Event, EventType::{
    ButtonPress, ButtonRelease, KeyPress, KeyRelease, MouseMove, Wheel
  }, Key
};

mod enum_mapping;
pub use crate::enum_mapping::*;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn js_display_size() -> Uint32Array {
  let (w, h) = display_size().unwrap();
  Uint32Array::new(vec![w as u32, h as u32])
}

#[napi]
pub fn mouse_move(x: f64, y: f64) {
  simulate(&MouseMove { x, y }).unwrap();
}

#[napi]
pub fn button_press(button: JsButton) {
  simulate(&ButtonPress(Button::from(button))).unwrap();
}

#[napi]
pub fn button_release(button: JsButton) {
  simulate(&ButtonRelease(Button::from(button))).unwrap();
}

#[napi]
pub fn wheel(delta_x: i64, delta_y: i64) {
  simulate(&Wheel{delta_x, delta_y}).unwrap();
}

#[napi]
pub fn key_press(key: JsKey) {
  simulate(&KeyPress(Key::from(key))).unwrap();
}

#[napi]
pub fn key_release(key: JsKey) {
  simulate(&KeyRelease(Key::from(key))).unwrap();
}
 
// #[napi(ts_args_type = "callback: (result: number) => void")]
// pub fn call_threadsafe_function(callback: JsFunction) -> Result<()> {
//   let tsfn: ThreadsafeFunction<Event, ErrorStrategy::Fatal> = callback
//     .create_threadsafe_function(0, |ctx| {
//       ctx.env.create_object().map(|v| vec![v])
//     })?;
//   fn callback_rust(event: Event) {
//     thread::spawn(move || {
//       tsfn.call(event.name, ThreadsafeFunctionCallMode::Blocking);
//     });
//   }
//   listen(callback_rust);
//   Ok(())
// }
