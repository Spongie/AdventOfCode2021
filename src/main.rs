#![allow(dead_code)]
use std::time::Instant;

mod common;
mod day1;
mod day2;

fn main() {
    let start = Instant::now();

    day1::solve_1();

    println!("Time: {}ms", start.elapsed().as_millis())
}
