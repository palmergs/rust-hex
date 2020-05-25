//#[derive(Debug, Copy, Clone, PartialEq)]
//pub enum Direction {
//    SOUTHEAST = 0,
//    NORTHEAST = 1,
//    NORTH = 2,
//    NORTHWEST = 3,
//    SOUTHWEST = 4,
//    SOUTH = 5,
//}

use std::f64::consts::PI;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Hex {
    pub q: i64,
    pub r: i64,
    pub s: i64,
}

static HEX_DIRECTIONS: [Hex; 6] = [
    Hex { q: 1, r: 0, s: -1 },
    Hex { q: 1, r: -1, s: 0 },
    Hex { q: 0, r: -1, s: 1 },
    Hex { q: -1, r: 0, s: 1 },
    Hex { q: -1, r: 1, s: 0 },
    Hex { q: 0, r: 1, s: -1 },
];

impl Hex {
    pub fn build(q: i64, r: i64, s: i64) -> Hex {
        assert!(q + r + s == 0);
        Hex { q: q, r: r, s: s }
    }

    pub fn add(&self, hex: &Hex) -> Hex {
        Hex::build(self.q + hex.q, self.r + hex.r, self.s + hex.s)
    }

    pub fn subtract(&self, hex: &Hex) -> Hex {
        Hex::build(self.q - hex.q, self.r - hex.r, self.s - hex.s)
    }

    pub fn multiply(&self, k: i64) -> Hex {
        Hex::build(self.q * k, self.r * k, self.s * k)
    }

    pub fn distance_origin(&self) -> i64 {
        (self.q.abs() + self.r.abs() + self.s.abs()) / 2
    }

    pub fn distance(&self, hex: &Hex) -> i64 {
        self.subtract(hex).distance_origin()
    }

    pub fn direction(dir: isize) -> &'static Hex {
        let dir = (6 + (dir % 6)) % 6;
        &HEX_DIRECTIONS[dir as usize]
    }

    pub fn neighbor(&self, dir: isize) -> Hex {
        Hex::direction(dir).add(self)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct FractionalHex {
    q: f64,
    r: f64,
    s: f64,
}

impl FractionalHex {
    fn build(q: f64, r: f64, s: f64) -> FractionalHex {
        FractionalHex{q: q, r: r, s: s}
    }
}


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
        f0: 3.0f64.sqrt(), f1: 3.0f64.sqrt() / 2.0f64, f2: 0.0, f3: 3.0f64 / 2.0f64,
        b0: 3.0f64.sqrt() / 3.0f64, b1: -1.0f64 / 3.0f64, b2: 0.0, b3: 2.0f64 / 3.0f64,
        start_angle: 0.5f64};

    static ref LAYOUT_FLAT: Orientation = Orientation{
        f0: 3.0f64 / 2.0f64, f1: 0.0, f2: 3.0f64.sqrt() / 2.0f64, f3: 3.0f64.sqrt(),
        b0: 2.0f64 / 3.0f64, b1: 0.0, b2: -1.0f64 / 3.0f64, b3: 3.0f64.sqrt() / 3.0f64,
        start_angle: 0.0};
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point(f64, f64);

pub struct Layout {
    orientation: &'static Orientation,
    size: Point,
    origin: Point,
}

impl Layout {
    pub fn build(orientation: &'static Orientation, size: Point, origin: Point) -> Layout {
        Layout{orientation: orientation, size: size, origin: origin}
    }

    pub fn to_pixel(&self, h: &Hex) -> Point {
        let m = self.orientation;
        let x = (m.f0 * h.q as f64 + m.f1 * h.r as f64) * self.size.0;
        let y = (m.f2 * h.q as f64 + m.f3 * h.r as f64) * self.size.1;
        Point(x + self.origin.0, y + self.origin.1)
    }

    pub fn to_hex(&self, p: &Point) -> FractionalHex {
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

    use super::{ Hex, HEX_DIRECTIONS, Layout, LAYOUT_POINTY, LAYOUT_FLAT, Point };

    #[test]
    fn direction_supports_any_value() {
        assert_eq!(*Hex::direction(0), HEX_DIRECTIONS[0]);    
        assert_eq!(*Hex::direction(6), HEX_DIRECTIONS[0]);    
        assert_eq!(*Hex::direction(-6), HEX_DIRECTIONS[0]);    
        assert_eq!(*Hex::direction(12), HEX_DIRECTIONS[0]);    
        assert_eq!(*Hex::direction(-12), HEX_DIRECTIONS[0]);    
    }

    #[test]
    fn build_layout_with_orientation() {
        let pointy = Layout::build(&LAYOUT_POINTY, Point(100.0, 100.0), Point(0.0, 0.0));
        let flat = Layout::build(&LAYOUT_FLAT, Point(100.0, 100.0), Point(0.0, 0.0));
    }
}

