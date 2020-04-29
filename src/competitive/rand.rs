use std::ops::{Add, Div, Mul, Rem, Sub};
use std::time::SystemTime;

// Linear Congruential Generator [Xn+1 = (A * Xn + B) Mod M]
pub struct LCG {
    a: u64,
    b: u64,
    m: u64,
    x: u64,
}
// let mut lcg = rand::LCG::new(1664525, 1013904223, 1 << 47, Some(0)); // Java's Constants
// let mut lcg = rand::LCG::new(6364136223846793005, 1, 1 << 63, None); // MMIX Donald Knuth

impl LCG {
    pub fn new(a: u64, b: u64, m: u64, seed: Option<u64>) -> Self {
        let x: u64 = match seed {
            Some(x) => x,
            None => match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                Ok(n) => n.as_secs(),
                Err(_) => 0,
            },
        };
        LCG { a, b, m, x }
    }

    pub fn next(&mut self) -> u64 {
        let x_new = (self.a.wrapping_mul(self.x).wrapping_add(self.b)) % self.m;
        self.x = x_new;
        x_new
    }
}

fn rol64(x: u64, k: i32) -> u64 {
    (x << k) | (x >> (64 - k))
}

#[derive(Debug)]
pub struct XoShiro256SS {
    s: [u64; 4],
}

impl XoShiro256SS {
    pub fn new(s: [u64; 4]) -> Self {
        Self { s }
    }

    pub fn next(&mut self) -> u64 {
        let result = rol64(self.s[1].wrapping_mul(5), 7).wrapping_mul(9);
        let t = self.s[1] << 17;

        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];

        self.s[2] ^= t;
        self.s[3] = rol64(self.s[3], 45);

        return result;
    }
}
