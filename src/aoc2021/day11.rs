use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;

type Gift = Board;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
struct Board {
    board: Vec<Vec<u8>>,
}

impl Board {
    fn get(&self, e: &(isize, isize)) -> u8 {
        if (0..self.len()).contains(&e.0) && (0..self.len()).contains(&e.1) {
            return self.board[e.0 as usize][e.1 as usize];
        }
        0
    }

    fn set(&mut self, e: &(isize, isize), val: u8) {
        if (0..self.len()).contains(&e.0) && (0..self.len()).contains(&e.1) {
            self.board[e.0 as usize][e.1 as usize] = val;
        }
    }

    fn len(&self) -> isize {
        self.board.len() as isize
    }

    fn get_adjacent(&self, e: &(isize, isize)) -> [(isize, isize); 8] {
        [
            (e.0 - 1, e.1 - 1), (e.0, e.1 - 1), (e.0 + 1, e.1 - 1), (e.0 - 1, e.1 + 1),
            (e.0, e.1 + 1), (e.0 + 1, e.1 + 1), (e.0 - 1, e.1), (e.0 + 1, e.1),
        ]
    }
}

#[aoc_generator(day11)]
fn read_input(input: &str) -> Gift {
    Board {
        board: input.lines().map(|line|
            line.bytes().map(|a| a - b'0').collect_vec()
        ).collect_vec()
    }
}

fn step(board: &mut Board) -> usize {
    board.board.iter_mut().flatten().for_each(|e| *e += 1);
    let mut marked: [[bool; 10]; 10] = Default::default();
    loop {
        let vv = (0..board.len()).cartesian_product(0..board.len())
            .filter(|x| board.get(x) > 9 && !marked[x.0 as usize][x.1 as usize]).collect_vec();
        if vv.is_empty() {
            break;
        }
        vv.iter().for_each(|v| marked[v.0 as usize][v.1 as usize] = true);
        vv.iter().map(|e| board.get_adjacent(e)).collect_vec().iter()
            .flatten().for_each(|i| board.set(&i, board.get(&i) + 1));
    }
    board.board.iter_mut().flatten().filter(|e| **e > 9).for_each(|e| *e = 0);
    marked.iter().flatten().filter(|x| **x).count()
}


#[aoc(day11, part1)]
fn day1x1(input: &Gift) -> usize {
    let mut board = input.clone();
    (0..100).map(|_| step(&mut board)).sum()
}

#[aoc(day11, part2)]
fn day1x2(input: &Gift) -> usize {
    let mut board = input.clone();
    let mut c = 1;
    while step(&mut board) != (board.len() * board.len()) as usize {
        c += 1;
    }
    c
}
