#![allow(dead_code)]
use std::time::Instant;

mod common;
mod day1;
mod day2;
mod day3;

fn main() {
    let start = Instant::now();

    day3::solve_2();

    println!("Time: {}ms", start.elapsed().as_millis())
}
