use std::collections::{HashMap, HashSet};
use std::detect::__is_feature_detected::sha;
use std::ops::Neg;
use std::process::exit;
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

type Gift = Vec<Region>;

const REGION: Region = Region {
    from: (-50, -50, -50),
    to: (50, 50, 50),
    state: false,
};

#[derive(Debug, Clone, Copy)]
struct Region {
    from: (i64, i64, i64),
    to: (i64, i64, i64),
    state: bool,
}

impl Region {
    fn intersect(&self, r2: &Self) -> Option<Self> {
        let overlap_x = (self.from.0..=self.to.0).filter(|h| (r2.from.0..=r2.to.0).contains(&h)).minmax().into_option()?;
        let overlap_y = (self.from.1..=self.to.1).filter(|h| (r2.from.1..=r2.to.1).contains(&h)).minmax().into_option()?;
        let overlap_z = (self.from.2..=self.to.2).filter(|h| (r2.from.2..=r2.to.2).contains(&h)).minmax().into_option()?;
        Some(Region {
            from: (overlap_x.0, overlap_y.0, overlap_z.0),
            to: (overlap_x.1, overlap_y.1, overlap_z.1),
            state: false,
        })
    }
    fn contains(&self, smaller: &Self) -> bool {
        (smaller.from.0..=smaller.to.0).all(|h| (self.from.0..=self.to.0).contains(&h)) &&
            (smaller.from.1..=smaller.to.1).all(|h| (self.from.1..=self.to.1).contains(&h)) &&
            (smaller.from.2..=smaller.to.2).all(|h| (self.from.2..=self.to.2).contains(&h))
    }

    fn volume(&self) -> i64 {
        (self.to.0 - self.from.0 + 1) * (self.to.1 - self.from.1 + 1) * (self.to.2 - self.from.2 + 1)
    }
}

#[aoc_generator(day22)]
fn read_input(input: &str) -> Gift {
    input.lines().map(|e| scan_fmt!(e,"{} x={}..{},y={}..{},z={}..{}",String,i64,i64,i64,i64,i64,i64)
        .unwrap()).map(|e|
        Region {
            state: e.0 == "on",
            from: (e.1, e.3, e.5),
            to: (e.2, e.4, e.6),
        }
    ).collect_vec()
}

fn calculate_sum(set: &Vec<(Region, bool)>) -> i64 {
    set.iter().fold(0, |acc, x| acc + if x.1 { 1 } else { -1 } * x.0.volume())
}

fn solve(input: &Vec<Region>) -> i64 {
    let mut set: Vec<(Region, bool)> = Vec::new();
    for reg in input.iter() {
        let mut v = set.iter().filter_map(|e| {
            if let Some(x) = e.0.intersect(reg) {
                return Some((x, !e.1));
            }
            None
        }).collect_vec();
        set.append(&mut v);
        if reg.state {
            set.push((*reg, true));
        }
    }
    calculate_sum(&set)
}

#[aoc(day22, part1)]
fn day2x1(input: &Gift) -> i64 {
    let input = input.iter().take(20).cloned().collect_vec();
    solve(&input)
}

#[aoc(day22, part2)]
fn day2x2(input: &Gift) -> i64 {
    solve(&input)
}
