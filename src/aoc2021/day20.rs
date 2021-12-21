use std::collections::HashSet;
use std::sync::mpsc::channel;
use std::thread;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use arrayvec::ArrayVec;
use bitvec::bits;
use bitvec::field::BitField;
use bitvec::order::Msb0;
use bitvec::prelude::BitSlice;
use bitvec::view::BitView;
use itertools::Itertools;

type Algorithm = ArrayVec<bool, 512>;
type Int = u32;
type Image = HashSet<(Int, Int)>;
type Border = (Int, Int, Int, Int);

type Gift = Input;

#[derive(Debug, Clone)]
struct Input {
    border_fill: bool,
    image: Image,
    algo: Algorithm,
}

#[aoc_generator(day20)]
fn read_input(input: &str) -> Gift {
    let (algo, image) = input.split_once("\n\n").unwrap();
    let algo = algo.chars().map(|e| e == '#').collect();
    let image = image.lines().enumerate().map(|e|
        e.1.chars().enumerate().filter(|e| e.1 == '#')
            .map(|k| (e.0 as Int + 100, k.0 as Int + 100)).collect_vec()).flatten().collect();
    Input {
        algo,
        image,
        border_fill: false,
    }
}

fn find_border(input: &Input) -> Border {
    let ((min_x, _), (max_x, _)) = input.image.iter().minmax_by_key(|e| e.0).into_option().unwrap();
    let ((_, min_y), (_, max_y)) = input.image.iter().minmax_by_key(|e| e.1).into_option().unwrap();
    (*min_x, *max_x, *min_y, *max_y)
}

fn apply_algorithm(input: &mut Input) {
    let border = find_border(input);
    let image: Image = (border.0 - 1..=border.1 + 1).cartesian_product(border.2 - 1..=border.3 + 1)
        .filter(|e| input.algo[find_index(input, e)] == input.border_fill).collect();
    input.border_fill = !input.border_fill;
    input.image = image;
}

fn find_index(input: &Input, pos: &(Int, Int)) -> usize {
    (pos.0 - 1..=pos.0 + 1).cartesian_product(pos.1 - 1..=pos.1 + 1)
        .map(|e| input.image.contains(&(e.0, e.1)) != input.border_fill)
        .fold(0, |acc, e| (acc << 1) | e as usize)
}

#[aoc(day20, part1)]
fn day2x1(input: &Gift) -> usize {
    let mut input = input.clone();
    for _ in 0..2 {
        apply_algorithm(&mut input);
    }
    input.image.len()
}

#[aoc(day20, part2)]
fn day2x2(input: &Gift) -> usize {
    let mut input = input.clone();
    for _ in 0..50 {
        apply_algorithm(&mut input);
    }
    input.image.len()
}
