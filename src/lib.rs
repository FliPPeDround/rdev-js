#![deny(clippy::all)]

use std::thread;
use std::time::UNIX_EPOCH;

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

#[napi(js_name = "displaySize", ts_return_type = "[width:number, height:number]")]
/// Returns the size in pixels of the main screen.
/// This is useful to use with x, y from MouseMove Event.
/// 
/// ### Example
/// ```javascript
/// import { displaySize } = from'@rdev-js/core';
/// 
/// const [width, height] = displaySize();
/// ```
/// 
/// @returns {[width:number, height:number]} The size in pixels of the main screen.
pub fn js_display_size() -> Uint32Array {
  let (w, h) = display_size().unwrap();
  Uint32Array::new(vec![w as u32, h as u32])
}

#[napi]
///  Move the mouse cursor to the specified x and y coordinates.
///
/// ### Example
/// ```javascript
/// import { mouseMove } from '@rdev-js/core';
/// 
/// mouseMove(100, 100);
/// ```
/// 
/// @param {number} x The x coordinate of the mouse.
/// @param {number} y The y coordinate of the mouse.
pub fn mouse_move(x: f64, y: f64) {
  simulate(&MouseMove { x, y }).unwrap();
}

#[napi]
/// simulate an individual mouse button press event.
/// 
/// ### Example
/// ```javascript
/// import { buttonPress, Button } from '@rdev-js/core';
/// 
/// buttonPress(Button.Left);
/// ```
/// 
/// @param {Button} button The button to press.
pub fn button_press(button: JsButton) {
  simulate(&ButtonPress(Button::from(button))).unwrap();
}

#[napi]
/// simulate an individual mouse button release event.
/// 
/// ### Example
/// ```javascript
/// import { buttonRelease, Button } from '@rdev-js/core';
/// 
/// buttonRelease(Button.Left);
/// ```
/// 
/// @param {Button} button The button to release.
pub fn button_release(button: JsButton) {
  simulate(&ButtonRelease(Button::from(button))).unwrap();
}

#[napi]
/// Simulate a scroll wheel event.
/// 
/// ### Example
/// ```javascript
/// import { wheel } from '@rdev-js/core';
/// 
/// wheel(0, 10);
/// ```
/// 
/// @param {number} deltaX represents horizontal scroll. Positive values correspond to scrolling right and negative values correspond to scrolling left.
/// @param {number} deltaY represents vertical scroll. Positive values correspond to scrolling down and negative values correspond to scrolling up.
pub fn wheel(delta_x: i64, delta_y: i64) {
  simulate(&Wheel{delta_x, delta_y}).unwrap();
}

#[napi]
/// Simulate a key press event.
/// 
/// ### Example
/// ```javascript
/// import { keyPress, Key } from '@rdev-js/core';
/// 
/// keyPress(Key.KeyA);
/// ```
/// 
/// @param {Key} key The key to press.
pub fn key_press(key: JsKey) {
  simulate(&KeyPress(Key::from(key))).unwrap();
}

#[napi]
/// Simulate a key release event.
/// 
/// ### Example
/// ```javascript
/// import { keyRelease, Key } from '@rdev-js/core';
/// 
/// keyRelease(Key.KeyA);
/// ```
/// 
/// @param {Key} key The key to release.
pub fn key_release(key: JsKey) {
  simulate(&KeyRelease(Key::from(key))).unwrap();
}

#[napi(
  js_name = "listen",
  ts_args_type = "
  callback: (
    event: {
      time: number,
      type: {
        direction: 'KeyPress' | 'KeyRelease'
        key: keyof typeof Key
      } | {
        direction: 'ButtonPress' | 'ButtonRelease'
        key: keyof typeof Button
      } | {
        direction: 'MouseMove'
        x: number,
        y: number
      } | {
        direction: 'Wheel'
        deltaX: number,
        deltaY: number
      },
      name?: string
    }
  ) => void
  ",
)]
/// Listening to global events. 
/// Caveat: On MacOS, you require the listen loop needs to be the primary app (no fork before) and need to have accessibility settings enabled.
/// 
/// ### Example
/// ```javascript
/// import { listen } from '@rdev-js/core';
/// 
/// listen((event) => {
///     console.log(event);
/// });
/// ```
/// 
/// @param {callback} callback The callback function to be called when an event occurs.
pub fn start_listening(callback: JsFunction) -> Result<()> {
    // Create a threadsafe function that can be called from another thread.
    let tsfn: ThreadsafeFunction<Event, ErrorStrategy::Fatal> = callback
        .create_threadsafe_function(0, |ctx| {
            let event: Event = ctx.value;
            let env = ctx.env;

            // Convert SystemTime to a duration since the UNIX_EPOCH
            // and then to milliseconds.
            let duration_since_epoch = match event.time.duration_since(UNIX_EPOCH) {
                Ok(duration) => duration,
                Err(_) => return Err(napi::Error::from_reason("Failed to convert time".to_owned())),
            };
            let time_in_ms = duration_since_epoch.as_millis();

            //Create the JavaScript object to pass to the callback.
            let mut js_event = env.create_object()?;
            js_event.set_named_property("time", env.create_int64(time_in_ms as i64)?)?;
            let event_type_js = match event.event_type {
                KeyPress(key) => {
                    let mut obj = env.create_object()?;
                    obj.set_named_property("direction", env.create_string("KeyPress")?)?;
                    obj.set_named_property("key", env.create_string(&format!("{:?}", key))?)?;
                    obj
                },
                KeyRelease(key) => {
                    let mut obj = env.create_object()?;
                    obj.set_named_property("direction", env.create_string("KeyRelease")?)?;
                    obj.set_named_property("key", env.create_string(&format!("{:?}", key))?)?;
                    obj
                },
                ButtonPress(button) => {
                    let mut obj = env.create_object()?;
                    obj.set_named_property("direction", env.create_string("ButtonPress")?)?;
                    obj.set_named_property("key", env.create_string(&format!("{:?}", button))?)?;
                    obj
                },
                ButtonRelease(button) => {
                    let mut obj = env.create_object()?;
                    obj.set_named_property("direction", env.create_string("ButtonRelease")?)?;
                    obj.set_named_property("key", env.create_string(&format!("{:?}", button))?)?;
                    obj
                },
                MouseMove { x, y } => {
                    let mut obj = env.create_object()?;
                    obj.set_named_property("direction", env.create_string("MouseMove")?)?;
                    obj.set_named_property("x", env.create_double(x)?)?;
                    obj.set_named_property("y", env.create_double(y)?)?;
                    obj
                },
                Wheel { delta_x, delta_y } => {
                    let mut obj = env.create_object()?;
                    obj.set_named_property("direction", env.create_string("Wheel")?)?;
                    obj.set_named_property("deltaX", env.create_int64(delta_x)?)?;
                    obj.set_named_property("deltaY", env.create_int64(delta_y)?)?;
                    obj
                },
            };
            js_event.set_named_property("type", event_type_js)?;

            if let Some(name) = event.name {
                js_event.set_named_property("name", env.create_string(&name)?)?;
            }

            //Return the JavaScript object in a vector as expected by the threadsafe function.
            Ok(vec![js_event])
        })?;
    // Spawn a new thread to listen for events.
    thread::spawn(move || {
        if let Err(error) = listen(move |event| {
          tsfn.call(event, ThreadsafeFunctionCallMode::NonBlocking);
        }) {
            eprintln!("Error listening to events: {:?}", error);
        }
    });

    Ok(())
}
