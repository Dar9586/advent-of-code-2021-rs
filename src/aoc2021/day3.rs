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

fn day3x2() -> i64 {
    let mut numbers0: Vec<String> = BufReader::new(File::open("./inputs/2021/day3.txt").expect("Missing input file")).lines().map(|x| x.unwrap()).collect();
    let mut numbers1: Vec<String> = numbers0.clone();

    for index in 0..12 {
        let one_count0 = numbers0.iter().filter(|element| element.chars().nth(index).unwrap() == '1').count();
        let one_count1 = numbers1.iter().filter(|element| element.chars().nth(index).unwrap() == '1').count();

        if numbers0.len() != 1 {
            numbers0 = numbers0.iter().filter(|el| el.chars().nth(index).unwrap() == if one_count0 >= (numbers0.len() - one_count0) { '1' } else { '0' }).map(|el| el.to_string()).collect();
        }
        if numbers1.len() != 1 {
            numbers1 = numbers1.iter().filter(|el| el.chars().nth(index).unwrap() == if one_count1 >= (numbers1.len() - one_count1) { '0' } else { '1' }).map(|el| el.to_string()).collect();
        }
    }
    i64::from_str_radix(numbers0.get(0).unwrap(), 2).unwrap() * i64::from_str_radix(numbers1.get(0).unwrap(), 2).unwrap()
}
