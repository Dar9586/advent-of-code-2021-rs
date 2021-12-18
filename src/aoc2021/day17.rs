use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

type Gift = Target;

const BRUTE_MAX: i64 = 200;

struct Target {
    from_x: i64,
    to_x: i64,
    from_y: i64,
    to_y: i64,
    valid_x_start: i64,
    valid_x_stop: i64,
}

impl Target {
    fn new(pos: (i64, i64, i64, i64)) -> Self {
        let valid_x_stop = (0..pos.1).fold_while(0, |acc, x|
            if acc + x > pos.1 { Done(x - 1) } else { Continue(acc + x) },
        ).into_inner();
        let valid_x_start = (0..pos.0).fold_while(0, |acc, x|
            if acc + x >= pos.0 { Done(x) } else { Continue(acc + x) },
        ).into_inner();
        Target {
            from_x: pos.0,
            to_x: pos.1,
            from_y: pos.2,
            to_y: pos.3,
            valid_x_start,
            valid_x_stop,
        }
    }

    fn contains(&self, pos: &(i64, i64)) -> bool {
        (self.from_x..=self.to_x).contains(&pos.0) &&
            (self.from_y..=self.to_y).contains(&pos.1)
    }
}

#[aoc_generator(day17)]
fn read_input(input: &str) -> Gift {
    Target::new(scan_fmt!(input,"target area: x={}..{}, y={}..{}",i64,i64,i64,i64).unwrap())
}

fn simulate_shoot(input: &Target, mut pos: (i64, i64)) -> i64 {
    let mut actual = (0, 0);
    let mut max_y = i64::MIN;
    while actual.1 > input.from_y && actual.0 < input.to_x {
        actual.0 += pos.0;
        actual.1 += pos.1;
        if input.contains(&actual) {
            return max_y;
        }
        if actual.1 > max_y {
            max_y = actual.1
        }
        pos.0 -= pos.0.signum();
        pos.1 -= 1;
    }
    i64::MIN
}

#[aoc(day17, part1)]
fn day2x1(input: &Gift) -> i64 {
    (input.valid_x_start..=input.valid_x_stop).cartesian_product(0..=BRUTE_MAX)
        .map(|y| simulate_shoot(input, y)).max().unwrap()
}

#[aoc(day17, part2)]
fn day2x2(input: &Gift) -> usize {
    (input.valid_x_start..=input.to_x).cartesian_product(-BRUTE_MAX..=BRUTE_MAX)
        .filter(|x| simulate_shoot(input, *x) != i64::MIN).count()
}
