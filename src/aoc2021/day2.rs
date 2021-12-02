use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn execute() {
    assert_eq!(day2x1(), 2073315);
    assert_eq!(day2x2(), 1840311528);
}

fn day2() -> (i64, i64, i64) {
    let file = BufReader::new(File::open("./inputs/2021/day2.txt").expect("Missing input file")).lines().map(|x| x.unwrap());
    let mut horizontal = 0i64;
    let mut depth = 0i64;
    let mut aim = 0i64;

    for line in file {
        let (command, amount) = scan_fmt!(line.as_str(),"{} {}",String,i64).unwrap();
        match command.as_str() {
            "forward" => {
                horizontal += amount;
                depth += amount * aim;
            }
            "down" => aim += amount,
            "up" => aim -= amount,
            _ => unreachable!()
        }
    }
    (horizontal, depth, aim)
}

fn day2x1() -> i64 {
    let (hor, _, depth) = day2();
    hor * depth
}

fn day2x2() -> i64 {
    let (hor, depth, _) = day2();
    hor * depth
}