#[macro_use] 
extern crate lazy_static;
extern crate regex;

mod solution;

fn main() {
    solution::day1::solve();
    solution::day2::solve();
    solution::day3::solve();
    solution::day4::solve();
    solution::day5::solve();
}
