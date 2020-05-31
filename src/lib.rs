#[macro_use]
extern crate lazy_static;

use std::f64;
use std::sync::atomic::{AtomicUsize,Ordering};
use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod axial;
mod cube;

static WEASEL_COUNT: AtomicUsize = AtomicUsize::new(0);

#[wasm_bindgen]
pub fn weasel() -> String {
  format!("there are {} weasels here", WEASEL_COUNT.fetch_add(1, Ordering::SeqCst))
}

fn window() -> web_sys::Window {
  web_sys::window().expect("no globale 'window' exists")
}

fn document() -> web_sys::Document {
  window().document().expect("should have a document on window")
}

fn now() -> f64 {
  window().performance().expect("should have a performance object").now()
}

fn element(id: &str) -> web_sys::Element {
  document().get_element_by_id(id).expect("should have a status element")
}

fn frame(f: &Closure<dyn FnMut()>) {
  window().request_animation_frame(f.as_ref().unchecked_ref()).expect("should get animation frame");
}

fn update_framerate() {
  let now = now();
  if let Some(last) = element("performance").get_attribute("timestamp") {
    if let Ok(n) = last.parse::<f64>() {
      let diff = (now - n) / 10 as f64;
      element("performance").set_text_content(Some(&format!("{}", diff)));
    }
  }
  element("performance").set_attribute("timestamp", &now.to_string()).unwrap();
}

#[wasm_bindgen]
pub fn start_loop() -> Result<(), JsValue> {
  let f = Rc::new(RefCell::new(None));
  let g = f.clone();
  let mut i = 0;

  *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
    if false {
      element("frame").set_text_content(Some("All done!"));

      // drop the handle to this content so that it will get cleaned up once we return.
      let _ = f.borrow_mut().take();
      return;
    }

    i += 1;
    element("frame").set_text_content(Some(&i.to_string()));
    if i % 10 == 0 {
      update_framerate();
    }

    
    frame(f.borrow().as_ref().unwrap());
  }) as Box<dyn FnMut()>));

  frame(g.borrow().as_ref().unwrap());
  Ok(())
}

#[wasm_bindgen]
pub fn smile() -> String {
  let doc = document();
  let can = doc.get_element_by_id("canvas").unwrap();
  let can: web_sys::HtmlCanvasElement = can.
    dyn_into::<web_sys::HtmlCanvasElement>().
    map_err(|_| ()).
    unwrap();

  let con = can.get_context("2d").
    unwrap().
    unwrap().
    dyn_into::<web_sys::CanvasRenderingContext2d>().
    unwrap();

  for n in 0..10 {
    draw_smile(&con, 75.0 + (n as f64 * 110.0), 75.0, 50.0);
    draw_smile(&con, 145.0 + (n as f64 * 100.0), 175.0, 40.0);
    draw_smile(&con, 200.0 + (n as f64 * 100.0), 220.0, 30.0);
  }


  "Did I smile?".to_string()
}

fn draw_smile(con: &web_sys::CanvasRenderingContext2d, x: f64, y: f64, r: f64) {
  con.begin_path();
  con.arc(x, y, r, 0.0, f64::consts::PI * 2.0).unwrap();
  
  con.move_to(x + r * 0.7, y);
  con.arc(x, y, r * 0.7, 0.0, f64::consts::PI).unwrap();

  con.move_to(x - r * 0.2, y - r * 0.2);
  con.arc(x - r * 0.2, y - r * 0.2, r * 0.1, 0.0, f64::consts::PI * 2.0).unwrap();
  
  con.move_to(x + r * 0.4, y - r * 0.2);
  con.arc(x + r * 0.3, y - r * 0.2, r * 0.1, 0.0, f64::consts::PI * 2.0).unwrap();
  
  con.stroke();
}

#[cfg(test)]
mod tests {
  use super::weasel;

  #[test]
  fn test_weasel() {
    assert_eq!(weasel(), "there are 0 weasels here");
    assert_eq!(weasel(), "there are 1 weasels here");
  }
}