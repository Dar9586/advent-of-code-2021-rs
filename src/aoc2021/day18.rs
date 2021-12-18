use std::fmt::Debug;
use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;

type Gift = Vec<Vec<Token>>;
#[derive(Debug, Clone, Copy)]
enum Token {
    Open,
    Close,
    Val(i32),
}
impl Token {
    fn value(&self) -> i32 {
        match self {
            Token::Val(x) => *x,
            _ => unreachable!()
        }
    }
    fn is_value(&self) -> bool {
        match self {
            Token::Val(_) => true,
            _ => false
        }
    }
}
#[aoc_generator(day18)]
fn read_input(input: &str) -> Gift {
    input.lines().map(|line| line.replace(",", "").chars().collect_vec()).map(|x| {
        x.iter().map(|x| match x {
            '[' => Token::Open,
            ']' => Token::Close,
            _ => Token::Val(x.to_string().parse().unwrap()),
        }).collect_vec()
    }).collect_vec()
}
fn add(left: &Vec<Token>, right: &Vec<Token>) -> Vec<Token> {
    std::iter::once(&Token::Open).chain(
        left.iter().chain(right.iter()).chain(
            std::iter::once(&Token::Close))).map(|e| e.clone()).collect_vec()
}
fn split(line: &mut Vec<Token>) -> bool {
    match line.iter().find_position(|e| e.is_value() && e.value() >= 10) {
        Some((pos, Token::Val(x))) => {
            line.splice(pos..=pos, vec![Token::Open, Token::Val(x / 2), Token::Val(x / 2 + x % 2), Token::Close]);
            true
        }
        _ => false
    }
}
fn explode(line: &mut Vec<Token>) -> bool {
    let mut depth = 0;
    for (pos, token) in line.iter().cloned().enumerate() {
        match token {
            Token::Open => {
                depth += 1;
                if depth == 5 {
                    let roba = line.splice(pos..pos + 4, std::iter::once(Token::Val(0))).skip(1).take(2).collect_vec();
                    if let Some(k) = line[..pos].iter().enumerate().filter(|e| e.1.is_value()).last() {
                        line.splice(k.0..=k.0, std::iter::once(Token::Val(k.1.value() + roba[0].value())));
                    }
                    if let Some(k) = line[pos + 1..].iter().enumerate().filter(|e| e.1.is_value()).next() {
                        line.splice(pos + 1 + k.0..=pos + 1 + k.0, std::iter::once(Token::Val(k.1.value() + roba[1].value())));
                    }
                    return true;
                }
            }
            Token::Close => { depth -= 1 }
            _ => {}
        }
    }
    false
}
fn magnitude(input: &mut Vec<Token>) -> usize {
    while input.len() != 1 {
        let (pos, (_, b, c, _)) = input.iter().tuple_windows().find_position(|(_, b, c, _)| b.is_value() && c.is_value()).unwrap();
        input.splice(pos..pos + 4, std::iter::once(Token::Val(b.value() * 3 + c.value() * 2)));
    }
    input[0].value() as usize
}
fn calculate(left: &Vec<Token>, right: &Vec<Token>) -> Vec<Token> {
    let mut a = add(left, right);
    loop {
        while explode(&mut a) {}
        if !split(&mut a) {
            return a;
        }
    }
}
#[aoc(day18, part1)]
fn day2x1(input: &Gift) -> usize {
    magnitude(&mut input.iter().cloned().reduce(|acc, x| calculate(&acc, &x)).unwrap())
}
#[aoc(day18, part2)]
fn day2x2(input: &Gift) -> usize {
    input.iter().cartesian_product(input.iter())
        .map(|v| magnitude(&mut calculate(v.0, v.1))).max().unwrap()
}
