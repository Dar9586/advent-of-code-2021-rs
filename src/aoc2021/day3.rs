use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn execute() {
    assert_eq!(day3x1(), 749376);
    assert_eq!(day3x2(), 2372923);
}

fn day3x1() -> i64 {
    let mut k = 0;
    BufReader::new(File::open("./inputs/2021/day3.txt").expect("Missing input file")).lines().map(|x| x.unwrap()).fold([0i64; 12], |mut acc, line| {
        for i in line.chars().enumerate() {
            acc[i.0] += i64::from(i.1 == '1');
        }
        k += 1;
        acc
    }).iter().fold([0, 0], |acc, el| [2 * acc[0] + i64::from(*el > k / 2), 2 * acc[1] + i64::from(*el < k / 2)]).iter().fold(1i64, |acc, y| acc * y)
}

fn day3x2_sub(most: bool) -> i64 {
    let mut values: Vec<String> = BufReader::new(File::open("./inputs/2021/day3.txt").expect("Missing input file")).lines().map(|x| x.unwrap()).collect();
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


fn day3x2() -> i64 {
    day3x2_sub(false) * day3x2_sub(true)
}
