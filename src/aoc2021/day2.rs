use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

type Gift = String;

#[aoc_generator(day2)]
fn read_input(input: &str) -> Vec<Gift> {
    input.lines().map(|e| e.to_string()).collect()
}


// Part 1 in SQL because why not
// SELECT (down-up)*fwd FROM (SELECT SUM(`value`) AS down FROM `day2` WHERE `command`="down") AS x0, (SELECT SUM(`value`) AS up FROM `day2` WHERE `command`="up") AS x1 , (SELECT SUM(`value`) AS fwd FROM `day2` WHERE `command`="forward") AS x2;

//Aim of part 2 is equivalent to Depth of part 1
fn day2(input: &[Gift]) -> (i64, i64, i64) {
    input.iter().fold((0i64, 0i64, 0i64), |mut acc, line| {
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

#[aoc(day2, part1)]
fn day2x1(input: &[Gift]) -> i64 {
    let (hor, _, depth) = day2(input);
    hor * depth
}

#[aoc(day2, part2)]
fn day2x2(input: &[Gift]) -> i64 {
    let (hor, depth, _) = day2(input);
    hor * depth
}