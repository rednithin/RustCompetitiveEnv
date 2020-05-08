#![allow(unused_imports)]

mod competitive;
use competitive::*;

use std::collections::{
    BTreeMap,   // Sorted Map
    BTreeSet,   // Sorted Set
    BinaryHeap, // Max Heap Implementation
    HashMap,    // Map
    HashSet,    // Set
    LinkedList, // Linked List not to be used anytime
    VecDeque,   // Deque
};

use std::cmp;
use std::io;
use std::str;
use std::time;

fn main() {
    let mut lcg = rand::LCG::new(1664525, 1013904223, 1 << 47, Some(0)); // Java's Constants
    let mut lcg = rand::LCG::new(6364136223846793005, 1, 1 << 63, None); // MMIX Donald Knuth
    let mut lcg = rand::XoShiro256SS::new(None); // XoShiro256SS
    let mut lcg = rand::XoShiro256SS::new(Some(10)); // XoShiro256SS

    let mut map: HashMap<u64, u64> = HashMap::new();

    for _ in 1..2000000 {
        let generated = lcg.next() % 10;
        if let Some(x) = map.get_mut(&generated) {
            *x = *x + 1;
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

    let mut scanner = util::Scanner::new(io::BufReader::new(std::io::stdin()));
    let vec = (0..10).map(|_| scanner.next()).collect::<Vec<u32>>();
    println!("{:?}", vec);
}
