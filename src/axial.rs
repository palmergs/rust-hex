#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Hex {
    pub q: i32,
    pub r: i32,
    s: i32,
}

impl Hex {
    pub fn build(q: i32, r: i32) -> Hex {
        let s: i32 = -q - r;
        assert!(q + r + s == 0);
        Hex { q: q, r: r, s: s }
    }
}
