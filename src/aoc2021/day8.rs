use std::collections::HashSet;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;
use lazy_static::lazy_static;

type SevenSegment = HashSet<char>;
type Gift = Entry;

lazy_static! {
    static ref ALL: HashSet<char> = ['a', 'b', 'c', 'd', 'e', 'f', 'g'].into();
}

#[derive(Clone, Debug)]
struct Entry {
    connections: Vec<SevenSegment>,
    output: Vec<SevenSegment>,
}

impl Entry {
    fn find(&self) -> Vec<&SevenSegment> {
        let mut init_pos = self.connections.iter().collect_vec();
        let mut true_pos = vec![init_pos[0]; 10];
        for uniq in [(1, 2), (7, 3), (4, 4), (8, 7)] {
            true_pos[uniq.0] = init_pos.remove(init_pos.iter().position(|x| x.len() == uniq.1).unwrap());
        }
        true_pos[6] = init_pos.remove(init_pos.iter().position(|x| x.len() == 6 && x.len() - x.difference(&true_pos[1]).count() == 1).unwrap());
        true_pos[0] = init_pos.remove(init_pos.iter().position(|x| x.len() == 6 && x.difference(&true_pos[4]).count() == 3).unwrap());
        true_pos[9] = init_pos.remove(init_pos.iter().position(|x| x.len() == 6).unwrap());
        let letter_c = ALL.difference(&true_pos[6]).next().unwrap();
        let letter_e = ALL.difference(&true_pos[9]).next().unwrap();
        true_pos[2] = init_pos.remove(init_pos.iter().position(|e| e.contains(letter_e)).unwrap());
        true_pos[3] = init_pos.remove(init_pos.iter().position(|e| e.contains(letter_c)).unwrap());
        true_pos[5] = init_pos[0];
        true_pos
    }
}

#[aoc_generator(day8)]
fn read_input(input: &str) -> Vec<Gift> {
    let mut entries = Vec::new();
    for line in input.lines() {
        let part = line.replace("|", "");
        let part = part.split_whitespace().collect_vec();
        entries.push(Entry {
            connections: Vec::from_iter(part[0..10].iter().map(|x| HashSet::from_iter(x.chars()))),
            output: Vec::from_iter(part[10..].iter().map(|x| HashSet::from_iter(x.chars()))),
        })
    }
    entries
}

#[aoc(day8, part1)]
fn day1x1(input: &[Gift]) -> usize {
    input.iter().map(|el| &el.output).flatten().filter(|x| (x.len() >= 2 && x.len() <= 4) || x.len() == 7).count()
}


#[aoc(day8, part2)]
fn day1x2(input: &[Gift]) -> usize {
    input.iter()
             .map(|x| (x.find(), &x.output)).fold(0, |acc, x|
                 acc + x.1.iter().fold(0, |acc, y|
                     10 * acc + x.0.iter().position(|n| n == &y).unwrap()),
             )
}
