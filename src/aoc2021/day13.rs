use std::collections::HashSet;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;

type Gift = Input;
type Coordinate = (i32, i32);
type Folds = (bool, i32);

#[derive(Clone, Eq, PartialEq, Debug)]
struct Input {
    dots: HashSet<Coordinate>,
    folds: Vec<Folds>,
}

#[aoc_generator(day13)]
fn read_input(input: &str) -> Gift {
    let parts = input.split_once("\n\n").unwrap();
    Input {
        dots: parts.0.lines().map(|e| e.split_once(",").unwrap())
            .map(|e| (e.0.parse().unwrap(), e.1.parse().unwrap())).collect(),
        folds: parts.1.lines().map(|e| e.split_once("=").unwrap())
            .map(|e| (e.0.contains("x"), e.1.parse().unwrap())).collect(),
    }
}

fn execute_fold(dots: &mut HashSet<Coordinate>, fold: &Folds) {
    if fold.0 {
        let folded = dots.drain_filter(|e| e.0 > fold.1)
            .map(|e| (e.0 - (e.0 - fold.1) * 2, e.1)).collect_vec();
        dots.extend(folded.iter());
    } else {
        let folded = dots.drain_filter(|e| e.1 > fold.1)
            .map(|e| (e.0, e.1 - (e.1 - fold.1) * 2)).collect_vec();
        dots.extend(folded.iter());
    };
}

fn print_grid(dots: &HashSet<Coordinate>) {
    for y in 0..6 {
        for x in 0..40 {
            print!("{}", if dots.contains(&(x, y)) { "█" } else { " " });
        }
        println!();
    }
}

#[aoc(day13, part1)]
fn day2x1(input: &Gift) -> usize {
    let mut i = input.clone();
    execute_fold(&mut i.dots, i.folds.iter().next().unwrap());
    i.dots.len()
}

#[aoc(day13, part2)]
fn day2x2(input: &Gift) -> i32 {
    let mut i = input.clone();
    i.folds.iter().for_each(|k| execute_fold(&mut i.dots, k));
    print_grid(&i.dots);
    0
}
