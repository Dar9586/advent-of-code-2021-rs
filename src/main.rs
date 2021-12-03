mod aoc2015;
mod aoc2021;
mod aoc2016;
#[macro_use] extern crate scan_fmt;
fn main() {
    aoc2021::day3::execute();
}

#[test]
fn test(){
    aoc2015::day1::execute();
    aoc2021::day1::execute();
    aoc2021::day2::execute();
    aoc2016::day1::execute();
}