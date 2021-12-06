use std::convert::TryInto;
use std::slice::Iter;
use std::str::FromStr;

use aoc_runner_derive::aoc;
use retain_mut::RetainMut;

#[derive(Default)]
struct Board {
    board: [[u64; 5]; 5],
    horizontal: [u8; 5],
    vertical: [u8; 5],
    sum: u64,
}

impl Board {
    fn from_reader(reader: &mut Iter<&str>) -> Self {
        let mut board = Board::default();
        for i in 0..5 {
            let vec: Vec<u64> = reader.next().unwrap().trim().split_whitespace().map(|x| u64::from_str(x).unwrap()).collect();
            board.sum += vec.iter().fold(0u64, |acc, val| acc + *val);
            board.board[i] = vec.try_into().unwrap();
        }
        board
    }
    fn extract_number(&mut self, val: u64) -> bool {
        for line in self.board.iter().enumerate() {
            for numb in line.1.iter().enumerate() {
                if numb.1 == &val {
                    self.horizontal[line.0] += 1;
                    self.vertical[numb.0] += 1;
                    self.sum -= val;
                    return self.horizontal[line.0] == 5 || self.vertical[numb.0] == 5;
                }
            }
        }
        false
    }
}


fn read_numbers(reader: &mut Iter<&str>) -> Vec<u64> {
    let vec: Vec<u64> = reader.next().unwrap().trim().split(',').map(|x| u64::from_str(x).unwrap()).collect();
    vec
}

fn read_data(input: &str) -> (Vec<u64>, Vec<Board>) {
    let lines: Vec<&str> = input.lines().filter(|x| !x.is_empty()).collect();
    let mut line = lines.iter();
    let mut boards: Vec<Board> = Vec::new();
    let numbers = read_numbers(&mut line);
    for _ in 0..100 {
        boards.push(Board::from_reader(&mut line));
    }
    (numbers, boards)
}

#[aoc(day4, part1)]
fn day4x1(input: &str) -> u64 {
    let (numbers, mut boards) = read_data(input);
    for val in numbers {
        for board in boards.as_mut_slice() {
            if board.extract_number(val) {
                return val * board.sum;
            }
        }
    }
    unreachable!()
}

#[aoc(day4, part2)]
fn day4x2(input: &str) -> u64 {
    let mut vec = Vec::new();
    let (numbers, mut boards) = read_data(input);
    for val in numbers {
        boards.retain_mut(|board| {
            if board.extract_number(val) {
                vec.push(val * board.sum);
                return false;
            }
            true
        });
    }
    vec.pop().unwrap()
}
