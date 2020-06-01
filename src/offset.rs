use super::{ Hex };

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Offset {
  pub col: i64,
  pub row: i64,
}

impl Offset {
  pub const EVEN: i64 = 1;
  pub const ODD: i64 = -1;

  pub fn new(col: i64, row: i64) -> Offset {
    Offset{ col, row }
  }

  pub fn qoffset(offset: i64, h: &Hex) -> Offset {
    assert!(offset == Offset::EVEN || offset == Offset::ODD);
    let col = h.q;
    let row = h.r + (h.q + offset * (h.q & 1)) / 2;
    Offset{ col, row }
  }

  pub fn roffset(offset: i64, h: &Hex) -> Offset {
    assert!(offset == Offset::EVEN || offset == Offset::ODD);
    let col = h.q + (h.r + offset * (h.r & 1)) / 2;
    let row = h.r;
    Offset{ col, row }
  }

  pub fn q_to_hex(&self, offset: i64) -> Hex {
    assert!(offset == Offset::EVEN || offset == Offset::ODD);
    let q = self.col;
    let r = self.row - (self.col + offset * (self.col & 1)) / 2;
    let s = -q - r;
    Hex{ q, r, s }
  }

  pub fn r_to_hex(&self, offset: i64) -> Hex {
    assert!(offset == Offset::EVEN || offset == Offset::ODD);
    let q = self.col - (self.row + offset * (self.row & 1)) / 2;
    let r = self.row;
    let s = -q - r;
    Hex { q, r, s }
  }
}

#[cfg(test)]
mod tests {
  use super::{Offset, Hex};

  #[test]
  fn offset_qroundtrip() {
    let hex = Hex::new(3, 4, -7);
    let noff = Offset::qoffset(Offset::EVEN, &hex);
    assert_eq!(hex, noff.q_to_hex(Offset::EVEN));

    let noff = Offset::qoffset(Offset::ODD, &hex);
    assert_eq!(hex, noff.q_to_hex(Offset::ODD));

    let offset = Offset::new(1, -3);
    let nhex = offset.q_to_hex(Offset::EVEN);
    assert_eq!(offset, Offset::qoffset(Offset::EVEN, &nhex));

    let nhex = offset.q_to_hex(Offset::ODD);
    assert_eq!(offset, Offset::qoffset(Offset::ODD, &nhex));
  }

  #[test]
  fn offset_rroundtrip() {
    let hex = Hex::new(3, 4, -7);
    let noff = Offset::roffset(Offset::EVEN, &hex);
    assert_eq!(hex, noff.r_to_hex(Offset::EVEN));

    let noff = Offset::roffset(Offset::ODD, &hex);
    assert_eq!(hex, noff.r_to_hex(Offset::ODD));    

    let offset = Offset::new(1, -3);
    let nhex = offset.r_to_hex(Offset::EVEN);
    assert_eq!(offset, Offset::roffset(Offset::EVEN, &nhex));

    let nhex = offset.r_to_hex(Offset::ODD);
    assert_eq!(offset, Offset::roffset(Offset::ODD, &nhex));
  }
}