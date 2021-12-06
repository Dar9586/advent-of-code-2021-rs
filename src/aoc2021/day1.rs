use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

type Gift = u32;

#[aoc_generator(day1)]
fn read_input(input: &str) -> Vec<Gift> {
    input.lines().map(|e| e.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
fn day1x1(input: &[Gift]) -> u32 {
    let mut old: Option<u32> = None;
    let mut increased = 0u32;

    for k in input {
        if let Some(val) = old {
            if *k > val {
                increased += 1;
            }
        }
        old = Some(*k);
    }
    increased
}


#[aoc(day1, part2)]
fn day1x2(input: &[Gift]) -> u32 {
    let mut increased = 0u32;
    let mut old: Option<u32> = None;

    for i in 0..input.len() - 2 {
        let sum = *(&input[i..i + 3].iter().sum());
        if let Some(val) = old {
            if sum > val {
                increased += 1;
            }
        }
        old = Some(sum)
    }
    increased
}