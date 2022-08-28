//! Use release mode
use rayon::prelude::*;
use std::time::Instant;

pub fn main() {
    let t = Instant::now();

    // Epic comment
    println!("Total: {:?}", t.elapsed());
}
