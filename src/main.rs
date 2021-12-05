#![allow(dead_code)]
use std::time::Instant;

mod common;
mod day4;

fn main() {
    let start = Instant::now();

    day4::solve_2();

    println!("Time: {}ms", start.elapsed().as_millis())
}
