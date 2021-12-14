use std::collections::HashMap;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;

type Cache = HashMap<(u8, u8, u8), HashMap<u8, usize>>;
type Gift = Formula;

#[derive(Clone, Debug)]
struct Formula {
    polymer: Vec<u8>,
    rules: HashMap<(u8, u8), u8>,
}

#[aoc_generator(day14)]
fn read_input(input: &str) -> Gift {
    let (polymer, remaining) = input.split_once("\n\n").unwrap();
    let mut rules = HashMap::new();
    remaining.lines().map(|line| line.as_bytes())
        .for_each(|v| { rules.insert((v[0], v[1]), v[6]); });
    Formula { polymer: polymer.as_bytes().to_vec(), rules }
}

fn step_rec(form: &(u8, u8), rules: &HashMap<(u8, u8), u8>, steps: u8, counts: &mut HashMap<u8, usize>, cache: &mut Cache) {
    if steps == 0 {
        counts.insert(form.1, counts.get(&form.1).unwrap_or(&0) + 1);
        return;
    }
    let res = *rules.get(form).unwrap();
    for t in [(form.0, res, steps - 1), (res, form.1, steps - 1)] {
        if !cache.contains_key(&t) {
            let mut sub_cache = HashMap::new();
            step_rec(&(t.0, t.1), rules, steps - 1, &mut sub_cache, cache);
            cache.insert(t, sub_cache);
        }
        cache.get(&t).unwrap().iter()
            .for_each(|v| *counts.entry(*v.0).or_insert(0) += v.1);
    }
}

fn diff(formula: &Gift, steps: u8) -> usize {
    let mut counts = HashMap::new();
    counts.insert(formula.polymer[0], 1);
    let mut cache = HashMap::new();
    for couple in formula.polymer.windows(2) {
        let mut v = HashMap::new();
        step_rec(&(couple[0], couple[1]), &formula.rules, steps, &mut v, &mut cache);
        v.iter().for_each(|k| *counts.entry(*k.0).or_insert(0) += k.1);
        cache.insert((couple[0], couple[1], steps), v);
    }
    let count = counts.values().minmax().into_option().unwrap();
    count.1 - count.0
}

#[aoc(day14, part1)]
fn day2x1(input: &Gift) -> usize {
    diff(input, 10)
}

#[aoc(day14, part2)]
fn day2x2(input: &Gift) -> usize {
    diff(input, 40)
}
