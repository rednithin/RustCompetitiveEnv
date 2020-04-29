mod lib;
use std::collections::HashMap;

fn main() {
    // let mut lcg = rand::LCG::new(1664525, 1013904223, 1 << 47, Some(0)); // Java's Constants
    let mut lcg = lib::rand::LCG::new(6364136223846793005, 1, 1 << 63, None); // MMIX Donald Knuth

    let mut map: HashMap<u64, u64> = HashMap::new();

    for _ in 1..20000 {
        let generated = lcg.next() % 10;
        if let Some(x) = map.get(&generated) {
            map.insert(generated, x + 1);
        } else {
            map.insert(generated, 1);
        }
    }

    println!("{:?}", map);
    for i in 0..10 {
        if let Some(x) = map.get(&i) {
            println!("{}: {}", i, x);
        } else {
            println!("{}: {}", i, 0);
        }
    }
}
