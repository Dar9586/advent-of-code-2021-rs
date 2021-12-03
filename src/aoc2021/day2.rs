use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn execute() {
    assert_eq!(day2x1(), 2073315);
    assert_eq!(day2x2(), 1840311528);
}

// Part 1 in SQL because why not
// SELECT (down-up)*fwd FROM (SELECT SUM(`value`) AS down FROM `day2` WHERE `command`="down") AS x0, (SELECT SUM(`value`) AS up FROM `day2` WHERE `command`="up") AS x1 , (SELECT SUM(`value`) AS fwd FROM `day2` WHERE `command`="forward") AS x2;

//Aim of part 2 is equivalent to Depth of part 1
fn day2() -> (i64, i64, i64) {
    BufReader::new(File::open("./inputs/2021/day2.txt").expect("Missing input file")).lines().map(|x| x.unwrap()).fold((0i64, 0i64, 0i64), |mut acc, line| {
        let (command, amount) = scan_fmt!(&line,"{} {}",String,i64).unwrap();
        match command.as_str() {
            "forward" => {
                acc.0 += amount;
                acc.1 += amount * acc.2;
            }
            "down" => acc.2 += amount,
            "up" => acc.2 -= amount,
            _ => unreachable!()
        }
        acc
    })
}

fn day2x1() -> i64 {
    let (hor, _, depth) = day2();
    hor * depth
}

fn day2x2() -> i64 {
    let (hor, depth, _) = day2();
    hor * depth
}