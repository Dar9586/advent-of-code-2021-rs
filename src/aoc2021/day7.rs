use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

type Gift = i64;

#[aoc_generator(day7)]
fn read_input(input: &str) -> Vec<Gift> {
    input.split(',').map(|e| e.parse().unwrap()).collect()
}

fn fuel_cost(input: &[Gift], x: &Gift) -> i64 {
    input.iter().map(|el| (el - x).abs()).sum()
}

fn fuel_cost_inc(input: &[Gift], x: &Gift) -> i64 {
    input.iter().map(|el| (1..=(el - x).abs()).sum::<i64>()).sum()
}

#[aoc(day7, part1)]
fn day1x1(input: &[Gift]) -> i64 {
    input.iter().map(|el| fuel_cost(input, el)).min().unwrap()
}


#[aoc(day7, part2)]
fn day1x2(input: &[Gift]) -> i64 {
    (0..=*input.iter().max().unwrap()).map(|el| fuel_cost_inc(input, &el)).min().unwrap()
}
