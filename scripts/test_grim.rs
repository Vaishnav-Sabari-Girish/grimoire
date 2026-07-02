#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! rand = "0.9"
//! ```

use rand::Rng;

fn main() {
    let mut rng = rand::rng();

    let n: u32 = rng.random();
    let m: u32 = rng.random_range(0..100);
    let k: u32 = rng.random_range(1..=100);

    println!("{n} {m} {k}");
}
