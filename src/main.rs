#[macro_use] 
extern crate lazy_static;
extern crate regex;

use std::time::{Duration, Instant};

mod solution;

fn main() {
    let now = Instant::now();
    solution::day1::solve();
    solution::day2::solve();
    solution::day3::solve();
    solution::day4::solve();
    solution::day5::solve();
    solution::day6::solve();
    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("{} seconds totally", sec);
}
