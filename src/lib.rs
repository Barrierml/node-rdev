#![deny(clippy::all)]

use napi_derive::napi;
use napi::{JsFunction, Result};
use rdev::{listen, EventType, Event};
use serde_json::{self, json};
use serde::Serialize;
use std::thread;
use napi::threadsafe_function::{ThreadsafeFunction, ErrorStrategy, ThreadsafeFunctionCallMode};

#[derive(Serialize)]
struct RdevEvent {
    event_type: String,
    name: Option<String>,
    time: std::time::SystemTime,
    data: String,
}

fn deal_event_to_json(event: Event) -> RdevEvent {
    let mut jsonify_event = RdevEvent {
        event_type: "".to_string(),
        name: event.name,
        time: event.time,
        data: "".to_string(),
    };
    match event.event_type {
        EventType::KeyPress(key) => {
            jsonify_event.event_type = "KeyPress".to_string();
            jsonify_event.data = json!({
                "key": format!("{:?}", key)
            }).to_string();
        }
        EventType::KeyRelease(key) => {
            jsonify_event.event_type = "KeyRelease".to_string();
            jsonify_event.data = json!({
                "key": format!("{:?}", key)
            }).to_string();
        }
        EventType::MouseMove { x, y } => {
            jsonify_event.event_type = "MouseMove".to_string();
            jsonify_event.data = json!({
                "x": x,
                "y": y
            }).to_string();
        }
        EventType::ButtonPress(key) => {
            jsonify_event.event_type = "ButtonPress".to_string();
            jsonify_event.data = json!({
                "key": format!("{:?}", key)
            }).to_string();
        }
        EventType::ButtonRelease(key) => {
            jsonify_event.event_type = "ButtonRelease".to_string();
            jsonify_event.data = json!({
                "key": format!("{:?}", key)
            }).to_string();
        }
        EventType::Wheel { delta_x, delta_y } => {
            jsonify_event.event_type = "Wheel".to_string();
            jsonify_event.data = json!({
                "delta_x": delta_x,
                "delta_y": delta_y
            }).to_string();
        }
    }

    jsonify_event
}

#[napi]
pub fn start_listener(callback: JsFunction) -> Result<()> {
  let tsfn: ThreadsafeFunction<String, ErrorStrategy::Fatal> = callback
    .create_threadsafe_function(0, |ctx| {
      let value = ctx.value;
      ctx.env.create_string_from_std(value).map(|v| vec![v])
    })?;

  thread::spawn(move || {
    if let Err(error) = listen(move |event| {
      let tsfn: ThreadsafeFunction<String, ErrorStrategy::Fatal> = tsfn.clone();
      let jsonify_event: RdevEvent = deal_event_to_json(event);
      // 转换成 JSON 格式
      let event_str: String = serde_json::to_string(&jsonify_event).unwrap();
      tsfn.call(event_str, ThreadsafeFunctionCallMode::Blocking);
    }) {
      eprintln!("Error: {:?}", error);
    }
  });

    Ok(())
}
