use std::f64::consts::PI;

use super::{FractionalHex, Hex};

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
    static ref LAYOUT_POINTY: Orientation = Orientation {
        f0: 3.0f64.sqrt(),
        f1: 3.0f64.sqrt() / 2.0,
        f2: 0.0,
        f3: 3.0 / 2.0,
        b0: 3.0f64.sqrt() / 3.0,
        b1: -1.0 / 3.0,
        b2: 0.0,
        b3: 2.0 / 3.0,
        start_angle: 0.5
    };
    static ref LAYOUT_FLAT: Orientation = Orientation {
        f0: 3.0 / 2.0,
        f1: 0.0,
        f2: 3.0f64.sqrt() / 2.0,
        f3: 3.0f64.sqrt(),
        b0: 2.0 / 3.0,
        b1: 0.0,
        b2: -1.0 / 3.0,
        b3: 3.0f64.sqrt() / 3.0,
        start_angle: 0.0
    };
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub const ORIGIN: Point = Point{ x: 0.0, y: 0.0 };

    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
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
        Layout { orientation, size, origin }
    }

    pub fn to_pixel(&self, h: &Hex) -> Point {
        let m = self.orientation;
        let x = (m.f0 * h.q as f64 + m.f1 * h.r as f64) * self.size.x;
        let y = (m.f2 * h.q as f64 + m.f3 * h.r as f64) * self.size.y;
        Point {
            x: x + self.origin.x,
            y: y + self.origin.y,
        }
    }

    pub fn to_fractional_hex(&self, p: &Point) -> FractionalHex {
        let m = self.orientation;
        let p2 = Point{ x: (p.x - self.origin.x) / self.size.x, y: (p.y - self.origin.y) / self.size.y };
        let q = m.b0 * p2.x + m.b1 * p2.y;
        let r = m.b2 * p2.x + m.b3 * p2.y;
        FractionalHex::build(q, r, -q - r)
    }

    pub fn polygon_corners(&self, h: &Hex) -> (Vec<Point>, Point) {
        let mut vec: Vec<Point> = Vec::new();
        let center = self.to_pixel(h);
        for i in 0..6 {
            let offset = self.hex_corner_offset(i);
            vec.push(Point {
                x: center.x + offset.x,
                y: center.y + offset.y,
            })
        }
        (vec, center)
    }

    fn hex_corner_offset(&self, corner: usize) -> Point {
        let angle = 2.0 * PI * (self.orientation.start_angle + corner as f64) / 6.0f64;
        Point {
            x: self.size.x * angle.cos(),
            y: self.size.y * angle.sin(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Hex, Layout, Point};

    #[test]
    fn build_layout_with_orientation() {
        let hex = Hex::new(3, 4, -7);

        let flat = Layout::flat(Point { x: 10.0, y: 15.0 }, Point { x: 35.0, y: 71.0 });
        assert_eq!(flat.to_fractional_hex(&flat.to_pixel(&hex)).round(), hex);

        let pointy = Layout::pointy(Point { x: 10.0, y: 15.0 }, Point { x: 35.0, y: 71.0 });
        assert_eq!(pointy.to_fractional_hex(&pointy.to_pixel(&hex)).round(), hex);
    }
}
