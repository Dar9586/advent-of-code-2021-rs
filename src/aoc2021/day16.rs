use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use bitvec::field::BitField;
use bitvec::order::Msb0;
use bitvec::prelude::BitVec;
use itertools::Itertools;

type Gift = Input;

#[derive(Clone, Debug)]
struct Input {
    data: BitVec<Msb0, u8>,
    pos: usize,
    version_sum: usize,
}

#[aoc_generator(day16)]
fn read_input(input: &str) -> Gift {
    Input::new(BitVec::from_vec(input.chars().chunks(2).into_iter().map(|k| {
        u8::from_str_radix(&k.collect::<String>(), 16).unwrap()
    }).collect::<Vec<u8>>()))
}

impl Input {
    fn new(data: BitVec<Msb0, u8>) -> Self {
        Input { data, pos: 0, version_sum: 0 }
    }

    fn parse(&mut self, count: usize) -> usize {
        let result = self.data[self.pos..self.pos + count].load_be();
        self.pos += count;
        result
    }
}


fn literal_value(input: &mut Input) -> usize {
    let mut val = 0;
    while input.parse(1) != 0 {
        val = val << 4 | input.parse(4);
    }
    val << 4 | input.parse(4)
}

fn unpack_operator(input: &mut Input) -> Vec<usize> {
    if input.parse(1) == 0 {
        let mut results = Vec::new();
        let target = input.parse(15) + input.pos;
        while input.pos != target {
            results.push(unpack(input).1);
        }
        return results;
    }
    (0..input.parse(11)).map(|_| unpack(input).1).collect()
}


fn unpack(input: &mut Input) -> (usize, usize) {
    let start = input.pos;
    input.version_sum += input.parse(3);
    let result = match input.parse(3) {
        0 => { unpack_operator(input).iter().sum() }
        1 => { unpack_operator(input).iter().product() }
        2 => { *unpack_operator(input).iter().min().unwrap() }
        3 => { *unpack_operator(input).iter().max().unwrap() }
        4 => { literal_value(input) }
        7 => { unpack_operator(input).iter().all_equal() as usize }
        5 => {
            let v = unpack_operator(input);
            (v[0] > v[1]) as usize
        }
        6 => {
            let v = unpack_operator(input);
            (v[0] < v[1]) as usize
        }
        _ => { unreachable!() }
    };
    (input.pos - start, result)
}

#[aoc(day16, part1)]
fn day2x1(input: &Gift) -> usize {
    let mut input = input.clone();
    unpack(&mut input);
    input.version_sum
}

#[aoc(day16, part2)]
fn day2x2(input: &Gift) -> usize {
    unpack(&mut input.clone()).1
}
