use std::fs::File;
use std::io::Read;

use text_io::try_read;

pub fn day1() {
    assert_eq!(day1x1(), 1624);
    assert_eq!(day1x2(), 1653);
}

fn day1x1() -> u32 {
    let mut file = File::open("./inputs/day1").expect("Missing input file").bytes().map(|ch| ch.unwrap());
    let mut old: Option<u32> = None;
    let mut increased = 0u32;

    while let Ok(k) = try_read!("{}",file) {
        if let Some(val) = old {
            if k > val {
                increased += 1;
            }
        }
        old = Some(k);
    }
    increased
}

fn day1x2() -> u32 {
    let mut file = File::open("./inputs/day1").expect("Missing input file").bytes().map(|ch| ch.unwrap());
    let mut vec: Vec<u32> = Vec::new();
    let mut increased = 0u32;
    let mut old: Option<u32> = None;

    while let Ok(k) = try_read!("{}",file) {
        vec.push(k);
    }

    for i in 0..vec.len() - 2 {
        let sum = *(&vec[i..i + 3].iter().sum());
        if let Some(val) = old {
            if sum > val {
                increased += 1;
            }
        }
        old = Some(sum)
    }
    increased
}