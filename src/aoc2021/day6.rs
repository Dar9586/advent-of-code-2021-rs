use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

pub fn execute() {
    assert_eq!(day6x1(), 373378);
    assert_eq!(day6x2(), 1682576647495);
}

fn read_input() -> Vec<usize> {
    let mut v = vec![0usize; 10];
    BufReader::new(File::open("./inputs/2021/day6.txt").unwrap()).lines()
        .next().unwrap().unwrap().split(",")
        .map(|e| e.parse::<u8>().unwrap()).zip(0..).into_group_map().iter()
        .for_each(|el| v[*el.0 as usize] = el.1.len());
    v
}

fn day6(day: u16) -> usize {
    let mut input = read_input();
    for _ in 0..day {
        let zero_day = input.remove(0);
        input[6] += zero_day;
        input[8] += zero_day;
        input.push(0);
    }
    input.iter().sum()
}

fn day6x1() -> usize {
    day6(80)
}

fn day6x2() -> usize {
    day6(256)
}