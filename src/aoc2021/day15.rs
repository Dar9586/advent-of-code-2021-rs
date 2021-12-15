use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;
use pathfinding::prelude::dijkstra;

type Gift = Maze;
type Position = (usize, usize);

#[derive(Clone, Debug)]
struct Maze {
    board: Vec<Vec<u8>>,
}

impl Maze {
    fn len(&self) -> usize {
        self.board.len()
    }

    fn get(&self, pos: Position) -> usize {
        let l = self.len();
        let p = self.board[pos.1 % l][pos.0 % l] as usize;
        let a = pos.0 / l + pos.1 / l;
        return ((p + a) % 10) + ((p + a) / 10);
    }
}

#[aoc_generator(day15)]
fn read_input(input: &str) -> Gift {
    Maze {
        board: input.lines().map(|line| line.as_bytes().iter().map(|c| c - b'0').collect_vec()).collect_vec()
    }
}

fn successors(input: &Maze, pos: &Position, max: usize) -> Vec<(Position, usize)> {
    [(pos.0 + 1, pos.1), (pos.0, pos.1 + 1), (pos.0 - 1, pos.1), (pos.0, pos.1 - 1)].iter()
        .filter(|p| (0..max).contains(&p.0) && (0..max).contains(&p.1))
        .map(|p| (*p, input.get(*p))).collect_vec()
}

#[aoc(day15, part1)]
fn day2x1(input: &Gift) -> usize {
    let l = input.len();
    dijkstra(&(0, 0), |p| successors(input, p, l), |p| *p == (l - 1, l - 1)).unwrap().1
}

#[aoc(day15, part2)]
fn day2x2(input: &Gift) -> usize {
    let l = input.len() * 5;
    dijkstra(&(0, 0), |p| successors(input, p, l), |p| *p == (l - 1, l - 1)).unwrap().1
}
