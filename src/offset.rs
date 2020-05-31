pub struct Offset {
  pub col: i32,
  pub row: i32,
}

impl Offset {
  pub const EVEN: i32 = 1;
  pub const ODD: i32 = -1;

  pub fn new(col: i32, row: i32) -> Offset {
    Offset{ col, row }
  }
}

#[cfg(test)]
mod tests {
  use super::{Offset};


}