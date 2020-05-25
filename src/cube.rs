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
    pub q: i32,
    pub r: i32,
    pub s: i32,
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
    pub fn build(q: i32, r: i32, s: i32) -> Hex {
        assert!(q + r + s == 0);
        Hex { q: q, r: r, s: s }
    }

    pub fn add(&self, hex: &Hex) -> Hex {
        Hex::build(self.q + hex.q, self.r + hex.r, self.s + hex.s)
    }

    pub fn subtract(&self, hex: &Hex) -> Hex {
        Hex::build(self.q - hex.q, self.r - hex.r, self.s - hex.s)
    }

    pub fn multiply(&self, k: i32) -> Hex {
        Hex::build(self.q * k, self.r * k, self.s * k)
    }

    pub fn distance_origin(&self) -> i32 {
        (self.q.abs() + self.r.abs() + self.s.abs()) / 2
    }

    pub fn distance(&self, hex: &Hex) -> i32 {
        self.subtract(hex).distance_origin()
    }
}
