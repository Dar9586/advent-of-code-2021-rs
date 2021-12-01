use std::fs::File;
use std::io::Read;

pub fn execute() {
    assert_eq!(day1x1(), 74);
    assert_eq!(day1x2(), 1795);
}

fn day1x1() -> i64 {
    let file = File::open("./inputs/2015/day1.txt").expect("Invalid input file").bytes().map(|x| x.unwrap());

    file.fold(0i64, |acc, x| acc + if x == b'(' { 1 } else { -1 })
}

fn day1x2() -> i64 {
    let file = File::open("./inputs/2015/day1.txt").expect("Invalid input file").bytes().map(|x| x.unwrap());
    let mut pos = 0;
    let mut floor = 0;

    for x in file {
        pos += 1;
        if x == b'(' {
            floor += 1;
        } else {
            floor -= 1;
            if floor < 0 {
                return pos;
            }
        }
    }
    unreachable!();
}
