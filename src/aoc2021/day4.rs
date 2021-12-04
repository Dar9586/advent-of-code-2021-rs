use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use retain_mut::RetainMut;

#[derive(Default)]
struct Board {
    board: [[u64; 5]; 5],
    horizontal: [u8; 5],
    vertical: [u8; 5],
    sum: u64,
}

impl Board {
    fn from_reader(reader: &mut BufReader<File>) -> Self {
        let mut board = Board::default();
        let mut line = String::new();
        for i in 0..5 {
            line.clear();
            reader.read_line(&mut line).unwrap();
            let vec: Vec<u64> = line.trim().split_whitespace().map(|x| u64::from_str(x).unwrap()).collect();
            board.sum += vec.iter().fold(0u64, |acc, val| acc + *val);
            board.board[i] = vec.try_into().unwrap();
        }
        reader.read_line(&mut line).unwrap();
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


pub fn execute() {
    assert_eq!(day4x1(), 55770);
    assert_eq!(day4x2(), 2980);
}

fn read_numbers(reader: &mut BufReader<File>) -> Vec<u64> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let vec: Vec<u64> = line.trim().split(',').map(|x| u64::from_str(x).unwrap()).collect();
    reader.read_line(&mut line).unwrap();
    vec
}

fn read_data() -> (Vec<u64>, Vec<Board>) {
    let mut reader = BufReader::new(File::open("./inputs/2021/day4.txt").unwrap());
    let mut boards: Vec<Board> = Vec::new();
    let numbers = read_numbers(&mut reader);
    for _ in 0..100 {
        boards.push(Board::from_reader(&mut reader));
    }
    (numbers, boards)
}

fn day4x1() -> u64 {
    let (numbers, mut boards) = read_data();
    for val in numbers {
        for board in boards.as_mut_slice() {
            if board.extract_number(val) {
                return val * board.sum;
            }
        }
    }
    unreachable!()
}


fn day4x2() -> u64 {
    let mut vec = Vec::new();
    let (numbers, mut boards) = read_data();
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
