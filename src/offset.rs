pub struct Offset {
  pub row: i64,
  pub col: i64,
}

impl Offset {
  pub fn new(col: i64, row: i64) -> Offset {
    Offset{ col, row }
  }
}