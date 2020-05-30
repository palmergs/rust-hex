#[macro_use]
extern crate lazy_static;

use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
// use web_sys::{window, HtmlCanvasElement, CanvasRenderingContext2d};

mod axial;
mod cube;

#[wasm_bindgen]
pub fn weasel() -> String {
  "there are so many weasels here".to_string()
}

#[wasm_bindgen]
pub fn smile() -> String {
  let doc = web_sys::window().unwrap().document().unwrap();
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
    assert_eq!(weasel(), "there are so many weasels here")
  }
}