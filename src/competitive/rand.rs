use std::time;

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
            None => match time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH) {
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

#[derive(Debug)]
pub struct XoShiro256SS {
    state: [u64; 4],
}

impl XoShiro256SS {
    pub fn new(seed: Option<u64>) -> Self {
        let seed: u64 = match seed {
            Some(x) => x,
            None => match time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH) {
                Ok(n) => n.as_secs(),
                Err(_) => 0,
            },
        };

        let mut splitmix64 = SplitMix64 { state: seed };
        let mut state = [0; 4];

        let tmp = splitmix64.next();
        state[0] = tmp;
        state[1] = tmp >> 32;

        let tmp = splitmix64.next();
        state[2] = tmp;
        state[3] = tmp >> 32;
        Self { state }
    }

    fn rol64(x: u64, k: i32) -> u64 {
        (x << k) | (x >> (64 - k))
    }

    pub fn next(&mut self) -> u64 {
        let result = XoShiro256SS::rol64(self.state[1].wrapping_mul(5), 7).wrapping_mul(9);
        let t = self.state[1] << 17;

        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];

        self.state[2] ^= t;
        self.state[3] = XoShiro256SS::rol64(self.state[3], 45);

        return result;
    }
}

struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    fn next(&mut self) -> u64 {
        let mut result: u64 = self.state;
        self.state = result.wrapping_add(0x9E3779B97f4A7C15);
        result = (result ^ (result >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        result = (result ^ (result >> 27)).wrapping_mul(0x94D049BB133111EB);
        return result ^ (result >> 31);
    }
}
