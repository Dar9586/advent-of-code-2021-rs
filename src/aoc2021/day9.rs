use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;

type Gift = Board;

#[derive(Clone)]
struct Board {
    board: Vec<Vec<u8>>,
}

impl Board {
    fn get(&self, e: &(isize, isize)) -> u8 {
        if e.0 < 0 || e.1 < 0 || e.0 >= self.len() || e.1 >= self.len() {
            return u8::MAX;
        }
        self.board[e.0 as usize][e.1 as usize]
    }

    fn mark(&mut self, e: &(isize, isize)) {
        self.board[e.0 as usize][e.1 as usize] = u8::MAX;
    }

    fn len(&self) -> isize {
        self.board.len() as isize
    }

    fn get_adjacent(&self, e: &(isize, isize)) -> [(isize, isize); 4] {
        [(e.0 + 1, e.1), (e.0 - 1, e.1), (e.0, e.1 + 1), (e.0, e.1 - 1)]
    }

    fn is_low(&self, pos: &(isize, isize), eq_ok: bool) -> bool {
        let pos_val = self.get(pos);
        self.get_adjacent(pos).iter().all(|adj| {
            let adj_val = self.get(adj);
            pos_val < adj_val || (eq_ok && pos_val == adj_val)
        })
    }
}

#[aoc_generator(day9)]
fn read_input(input: &str) -> Gift {
    Board {
        board:
        input.lines().
            map(|line| line.split("").filter(|x| !x.is_empty())
                .map(|e| e.parse::<u8>().unwrap()).collect_vec()
            ).collect_vec()
    }
}

fn find_local_mins(board: &Board) -> Vec<(isize, isize)> {
    (0..board.len()).cartesian_product(0..board.len())
        .filter(|e| board.is_low(e, false)).collect_vec()
}

fn find_basin(board: &mut Board, low: &(isize, isize)) -> usize {
    let mut v = vec![low.clone()];
    let mut basin_size = 0;
    while let Some(pos) = v.pop() {
        if board.is_low(&pos, true) && board.get(&pos) < 9 {
            basin_size += 1;
            v.append(&mut board.get_adjacent(&pos).to_vec());
            board.mark(&pos);
        }
    }
    basin_size
}

#[aoc(day9, part1)]
fn day1x1(input: &Gift) -> usize {
    find_local_mins(input).iter()
        .fold(0, |acc, e| acc + 1 + input.get(e) as usize)
}

#[aoc(day9, part2)]
fn day1x2(input: &Gift) -> usize {
    let mut input = input.clone();
    let mut basins = find_local_mins(&input).iter()
        .map(|el| find_basin(&mut input, el)).collect_vec();
    basins.sort();
    basins[basins.len() - 3..].iter().product()
}
