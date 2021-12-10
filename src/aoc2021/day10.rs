use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;

type Gift = Vec<u8>;

const OPENING: [u8; 4] = [b'(', b'[', b'{', b'<', ];
const CLOSING: [u8; 4] = [b')', b']', b'}', b'>'];
const POINTS1: [usize; 4] = [3, 57, 1197, 25137];

#[aoc_generator(day10)]
fn read_input(input: &str) -> Vec<Gift> {
    input.lines().map(|line| line.bytes().collect_vec()).collect_vec()
}


fn find_error(line: &Vec<u8>) -> Result<Vec<&u8>, &u8> {
    let mut stack = Vec::new();
    for item in line {
        if OPENING.contains(item) {
            stack.push(item)
        } else {
            let k = stack.pop().unwrap();
            if item != &CLOSING[OPENING.iter().position(|x| x == k).unwrap()] {
                return Err(item);
            }
        }
    }
    Ok(stack)
}

#[aoc(day10, part1)]
fn day1x1(input: &Vec<Gift>) -> usize {
    input.iter().filter_map(|e| find_error(e).err())
        .map(|val| POINTS1[CLOSING.iter().position(|x| x == val).unwrap()] as usize)
        .sum()
}

#[aoc(day10, part2)]
fn day1x2(input: &Vec<Gift>) -> usize {
    let v = input.iter().filter_map(|e| find_error(e).ok())
        .map(|stack| stack.iter().rev()
            .fold(0, |acc, el| 5 * acc + OPENING.iter().position(|x| &x == el).unwrap() + 1))
        .sorted().collect_vec();
    v[v.len() / 2]
}
