use std::f64::consts::PI;

use super::{ Hex, FractionalHex };

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Orientation {
    f0: f64,
    f1: f64,
    f2: f64,
    f3: f64,
    b0: f64,
    b1: f64,
    b2: f64,
    b3: f64,
    start_angle: f64,
}

lazy_static! {
    static ref LAYOUT_POINTY: Orientation = Orientation{
        f0: 3.0f64.sqrt(), f1: 3.0f64.sqrt() / 2.0, f2: 0.0, f3: 3.0 / 2.0,
        b0: 3.0f64.sqrt() / 3.0, b1: -1.0 / 3.0, b2: 0.0, b3: 2.0 / 3.0,
        start_angle: 0.5};

    static ref LAYOUT_FLAT: Orientation = Orientation{
        f0: 3.0 / 2.0, f1: 0.0, f2: 3.0f64.sqrt() / 2.0, f3: 3.0f64.sqrt(),
        b0: 2.0 / 3.0, b1: 0.0, b2: -1.0 / 3.0, b3: 3.0f64.sqrt() / 3.0,
        start_angle: 0.0};
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point(f64, f64);

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point(x, y)
    }
}

#[derive(Debug)]
pub struct Layout {
    orientation: &'static Orientation,
    size: Point,
    origin: Point,
}

impl Layout {
    pub fn pointy(size: Point, origin: Point) -> Layout {
        Layout::build(&LAYOUT_POINTY, size, origin)
    }

    pub fn flat(size: Point, origin: Point) -> Layout {
        Layout::build(&LAYOUT_FLAT, size, origin)
    }

    pub fn build(orientation: &'static Orientation, size: Point, origin: Point) -> Layout {
        Layout{orientation: orientation, size: size, origin: origin}
    }

    pub fn to_pixel(&self, h: &Hex) -> Point {
        let m = self.orientation;
        let x = (m.f0 * h.q as f64 + m.f1 * h.r as f64) * self.size.0;
        let y = (m.f2 * h.q as f64 + m.f3 * h.r as f64) * self.size.1;
        Point(x + self.origin.0, y + self.origin.1)
    }

    pub fn to_fractional_hex(&self, p: &Point) -> FractionalHex {
        let m = self.orientation;
        let q = m.b0 * p.0 + m.b1 * p.1;
        let r = m.b2 * p.0 + m.b3 * p.1;
        FractionalHex::build(q, r, -q - r)
    }

    pub fn polygon_corners(&self, h: &Hex) -> Vec<Point> {
        let mut vec: Vec<Point> = Vec::new();
        let center = self.to_pixel(h);
        for i in 0..6 {
            let offset = self.hex_corner_offset(i);
            vec.push(Point(center.0 + offset.0, center.1 + offset.1))
        }
        vec
    }

    fn hex_corner_offset(&self, corner: usize) -> Point {
        let angle = 2.0 * PI * (self.orientation.start_angle + corner as f64) / 6.0f64;
        Point(self.size.0 * angle.cos(), self.size.1 * angle.sin())
    }
}

#[cfg(test)]
mod tests {
  use super::{ Point, Layout, LAYOUT_POINTY, LAYOUT_FLAT };

  #[test]
  fn build_layout_with_orientation() {
      let pointy = Layout::build(&LAYOUT_POINTY, Point(100.0, 100.0), Point(0.0, 0.0));
      let flat = Layout::build(&LAYOUT_FLAT, Point(100.0, 100.0), Point(0.0, 0.0));
  }
}