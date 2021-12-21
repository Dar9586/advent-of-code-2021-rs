use std::collections::{HashMap, HashSet};
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

const DICE_FACES: u16 = 100;
const CELLS: u16 = 10;

const QUANTUM_POINTS: u16 = 21;
const QUANTUM_DICE_FACES: u16 = 3;

type Cache = HashMap<(u8, u8, u16, u16, u16, bool), (u64, u64)>;

#[derive(Debug, Clone, Copy)]
struct Input {
    pos1: u8,
    pos2: u8,
    points1: u16,
    points2: u16,
    dice_val: u16,
    roll_count: u16,
    turn_pos1: bool,
}

impl Input {
    fn make_roll(&mut self) -> u16 {
        self.dice_val += 1;
        self.roll_count += 1;
        if self.dice_val > DICE_FACES {
            self.dice_val = 1;
        }
        self.dice_val
    }
    fn update(&mut self, vals: (u16, u16, u16)) {
        let pos = if self.turn_pos1 { self.pos1 } else { self.pos2 } as u16;
        let cells = pos + vals.0 + vals.1 + vals.2;
        if self.turn_pos1 {
            self.pos1 = (cells % CELLS) as u8;
            self.points1 += (self.pos1 + 1) as u16;
        } else {
            self.pos2 = (cells % CELLS) as u8;
            self.points2 += (self.pos2 + 1) as u16;
        }
        self.turn_pos1 = !self.turn_pos1;
    }
}

#[aoc_generator(day21)]
fn read_input(input: &str) -> Gift {
    let input = input.split_once("\n").unwrap();
    Input {
        pos1: scan_fmt!(input.0,"Player 1 starting position: {}",u8).unwrap() - 1,
        pos2: scan_fmt!(input.1,"Player 2 starting position: {}",u8).unwrap() - 1,
        dice_val: 0,
        turn_pos1: true,
        points1: 0,
        points2: 0,
        roll_count: 0,
    }
}


#[aoc(day21, part1)]
fn day2x1(input: &Gift) -> u32 {
    let mut input = input.clone();
    while input.points1 < 1000 && input.points2 < 1000 {
        let v = (input.make_roll(), input.make_roll(), input.make_roll());
        input.update(v);
    }
    input.points1.min(input.points2) as u32 * input.roll_count as u32
}

fn quantum_dice_roll(input: &mut Input, victory: &mut (u64, u64), cache: &mut Cache) {
    for v in (1..=QUANTUM_DICE_FACES).cartesian_product(1..=QUANTUM_DICE_FACES)
        .cartesian_product(1..=QUANTUM_DICE_FACES).map(|e| (e.0.0, e.0.1, e.1)) {
        let mut k = input.clone();
        k.update(v);
        if k.points1 >= QUANTUM_POINTS {
            victory.0 += 1;
            continue;
        } else if k.points2 >= QUANTUM_POINTS {
            victory.1 += 1;
            continue;
        }
        let state = (k.pos1, k.pos2, k.dice_val, k.points1, k.points2, k.turn_pos1);
        if !cache.contains_key(&state) {
            let mut vv = (0, 0);
            quantum_dice_roll(&mut k, &mut vv, cache);
            cache.insert(state, vv);
        }
        let vv = cache.get(&state).unwrap();
        victory.0 += vv.0;
        victory.1 += vv.1;
    }
}

#[aoc(day21, part2)]
fn day2x2(input: &Gift) -> u64 {
    let mut victory = (0, 0);
    quantum_dice_roll(&mut input.clone(), &mut victory, &mut Cache::new());
    victory.0.max(victory.1)
}
