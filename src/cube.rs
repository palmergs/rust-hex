//#[derive(Debug, Copy, Clone, PartialEq)]
//pub enum Direction {
//    SOUTHEAST = 0,
//    NORTHEAST = 1,
//    NORTH = 2,
//    NORTHWEST = 3,
//    SOUTHWEST = 4,
//    SOUTH = 5,
//}

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
    pub const ORIGIN: Hex = Hex{ q: 0, r: 0, s: 0 };

    pub fn new(q: i64, r: i64, s: i64) -> Hex {
        assert!(q + r + s == 0);
        Hex::build(q, r, s)
    }

    pub fn axial(q: i64, r: i64) -> Hex {
        let s: i64 = -q - r;
        assert!(q + r + s == 0);
        Hex::build(q, r, s)
    } 
    
    pub fn build(q: i64, r: i64, s: i64) -> Hex {
        Hex { q, r, s }
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

    pub fn rotate_left(&self) -> Hex {
        Hex::build(-self.s, -self.q, -self.r)
    }

    pub fn rotate_right(&self) -> Hex {
        Hex::build(-self.r, -self.s, -self.q)
    }

    pub fn fractional(&self) -> FractionalHex {
        FractionalHex::new(self.q as f64, self.r as f64, self.s as f64)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FractionalHex {
    q: f64,
    r: f64,
    s: f64,
}

impl FractionalHex {
    pub fn new(q: f64, r: f64, s: f64) -> FractionalHex {
        FractionalHex::build(q, r, s)
    }

    pub fn build(q: f64, r: f64, s: f64) -> FractionalHex {
        FractionalHex{ q, r, s }
    }

    pub fn round(&self) -> Hex {
        let mut q = self.q.round() as i64; // self.q.round() as i64;
        let mut r = self.r.round() as i64;
        let mut s = self.s.round() as i64;
        let q_diff = (q as f64 - self.q).abs();
        let r_diff = (r as f64 - self.r).abs();
        let s_diff = (s as f64 - self.s).abs();
        if q_diff > r_diff && q_diff > s_diff {
            q = -r - s;
        } else if r_diff > s_diff {
            r = -q -s;
        } else {
            s = -q - r;
        }
        return Hex{ q, r, s }
    }
}

#[cfg(test)]
mod tests {
    use super::{ Hex, FractionalHex, HEX_DIRECTIONS };

    #[test]
    fn arithmetic() {
        assert_eq!(Hex::new(4, -10, 6), Hex::new(1, -3, 2).add(&Hex::new(3, -7, 4)));
        assert_eq!(Hex::new(-2, 4, -2), Hex::new(1, -3, 2).subtract(&Hex::new(3, -7, 4)));
    }

    #[test]
    fn direction() {
        assert_eq!(Hex::new(0, -1, 1), *Hex::direction(2));
    }

    #[test]
    fn neighbor() {
        assert_eq!(Hex::new(1, -3, 2), Hex::new(1, -2, 1).neighbor(2));
    }

    #[test]
    fn distance() {
        assert_eq!(7, Hex::new(3, -7, 4).distance(&Hex::ORIGIN))
    }

    #[test]
    fn rotate_right() {
        assert_eq!(Hex::new(1, -3, 2).rotate_right(), Hex::new(3, -2, -1));
    }

    #[test]
    fn rotate_left() {
        assert_eq!(Hex::new(1, -3, 2).rotate_left(), Hex::new(-2, -1, 3));
    }

    #[test]
    fn fractional_round_trip() {
        let hex = Hex::new(4, -10, 6);
        assert_eq!(hex, hex.fractional().round());
    }

    #[test]
    fn direction_supports_any_value() {
        assert_eq!(*Hex::direction(0), HEX_DIRECTIONS[0]);    
        assert_eq!(*Hex::direction(6), HEX_DIRECTIONS[0]);    
        assert_eq!(*Hex::direction(-6), HEX_DIRECTIONS[0]);    
        assert_eq!(*Hex::direction(12), HEX_DIRECTIONS[0]);    
        assert_eq!(*Hex::direction(-12), HEX_DIRECTIONS[0]);    
    }
}

