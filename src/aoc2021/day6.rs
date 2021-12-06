use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;

type Gift = usize;

#[aoc_generator(day6)]
fn read_input(input: &str) -> Vec<Gift> {
    let mut v = vec![0usize; 10];
    input.split(",")
        .map(|e| e.parse::<u8>().unwrap()).zip(0..).into_group_map().iter()
        .for_each(|el| v[*el.0 as usize] = el.1.len());
    v
}

fn day6(input: &[Gift], day: u16) -> usize {
    let mut input = input.to_vec();
    for _ in 0..day {
        let zero_day = input.remove(0);
        input[6] += zero_day;
        input[8] += zero_day;
        input.push(0);
    }
    input.iter().sum()
}


#[aoc(day6, part1)]
fn day6x1(input: &[Gift]) -> usize {
    day6(input, 80)
}

#[aoc(day6, part2)]
fn day6x2(input: &[Gift]) -> usize {
    day6(input, 256)
}
