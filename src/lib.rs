#[macro_use]
extern crate lazy_static;

use std::f64;
use std::sync::atomic::{AtomicUsize,Ordering};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant, UNIX_EPOCH};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod axial;

mod cube;
use cube::{ Hex, FractionalHex };

mod offset;
use offset::{ Offset };

mod layout;
use layout::{ Layout, Point };

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

fn body() -> web_sys::HtmlElement {
  document().body().expect("document should have a body")
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

#[wasm_bindgen]
pub fn start_loop() -> Result<(), JsValue> {
  let f = Rc::new(RefCell::new(None));
  let g = f.clone();
  let mut i = 0;
  let performance = window().performance().expect("performance should be avilable");
  
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
      let avg = performance.now() / i as f64;
      element("performance").set_text_content(Some(&format!("{:.2}", avg)));
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

#[wasm_bindgen]
pub fn draw_hexes() -> String {
  let width = body().client_width() as f64;
  let height = body().client_height() as f64;
  let size = 15f64;

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

  con.set_stroke_style(&JsValue::from("rgba(100, 100, 100, 0.5)"));
  con.set_fill_style(&JsValue::from("rgba(100, 100, 255, 0.5)"));
  con.set_line_width(0.25);
  con.set_font("7px sans-serif");
  let layout = Layout::flat(Point::new(size, size), Point::ORIGIN);
  let cols = width / size;
  let rows = height / size;
  for col in 0..cols as i64 {
    for row in 0..rows as i64 {
      let hex = match row % 2 {
        0 => Offset::q_to_hex(col, row, Offset::EVEN),
        1 => Offset::q_to_hex(col, row, Offset::ODD),
        _ => panic!("srsly? how did mod integer"),
      };
      draw_hex(&con, &layout, &hex);
    }
  }
  

  "Done!".to_string()
}

fn draw_hex(con: &web_sys::CanvasRenderingContext2d, layout: &Layout, hex: &Hex) {
  
  let corners = layout.polygon_corners(&hex);
  con.fill_text(&hex.q.to_string(), corners[0].x + 5.0f64, corners[0].y - 18.0f64).unwrap();
  con.fill_text(&hex.r.to_string(), corners[2].x + 9.0f64, corners[2].y - 3.0f64).unwrap();
  con.fill_text(&hex.s.to_string(), corners[4].x - 3.0f64, corners[4].y - 3.0f64).unwrap();

  con.begin_path();  
  con.move_to(corners[0].x, corners[0].y);
  con.line_to(corners[1].x, corners[1].y);
  con.line_to(corners[2].x, corners[2].y);
  con.line_to(corners[3].x, corners[3].y);
  con.line_to(corners[4].x, corners[4].y);
  con.line_to(corners[5].x, corners[5].y);
  con.line_to(corners[0].x, corners[0].y);
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