#[macro_use]
extern crate lazy_static;

use wasm_bindgen::prelude::*;

mod axial;
mod cube;

#[wasm_bindgen]
pub fn weasel() -> String {
  "there are so many weasels here".to_string()
}

#[cfg(test)]
mod tests {
  use super::weasel;

  #[test]
  fn test_weasel() {
    assert_eq!(weasel(), "there are so many weasels here")
  }
}