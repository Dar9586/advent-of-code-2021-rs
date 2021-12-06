use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

type Gift = String;

#[aoc_generator(day3)]
fn read_input(input: &str) -> Vec<Gift> {
    input.lines().map(|e| e.to_string()).collect()
}

#[aoc(day3, part1)]
fn day3x1(input: &[Gift]) -> i64 {
    let mut k = 0;
    input.iter().fold([0i64; 12], |mut acc, line| {
        for i in line.chars().enumerate() {
            acc[i.0] += i64::from(i.1 == '1');
        }
        k += 1;
        acc
    }).iter().fold([0, 0], |acc, el| [2 * acc[0] + i64::from(*el > k / 2), 2 * acc[1] + i64::from(*el < k / 2)]).iter().fold(1i64, |acc, y| acc * y)
}

fn day3x2_sub(input: &[Gift], most: bool) -> i64 {
    let mut values: Vec<String> = input.to_vec();
    let mut index = 0;
    while values.len() != 1 {
        let one_count = values.iter().filter(|element| element.chars().nth(index).unwrap() == '1').count();
        let len = values.len();
        if most {
            values.retain(|el| el.chars().nth(index).unwrap() == if one_count >= (len - one_count) { '1' } else { '0' });
        } else {
            values.retain(|el| el.chars().nth(index).unwrap() == if one_count >= (len - one_count) { '0' } else { '1' });
        }
        index += 1;
    }
    i64::from_str_radix(values.get(0).unwrap(), 2).unwrap()
}

#[aoc(day3, part2)]
fn day3x2(input: &[Gift]) -> i64 {
    day3x2_sub(input, false) * day3x2_sub(input, true)
}
