use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;

type Gift = Pipe;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: u16,
    y: u16,
}

#[derive(Debug, Copy, Clone)]
struct Pipe {
    from: Point,
    to: Point,
}

impl Point {
    fn new(x: u16, y: u16) -> Self {
        Point { x, y }
    }
}

impl Pipe {
    fn from_coord(coords: (u16, u16, u16, u16)) -> Self {
        Pipe{
            from: Point::new(coords.0,coords.1),
            to: Point::new(coords.2,coords.3)
        }
    }
    fn points(&self)->Vec<Point>{
        let mut vec=Vec::new();
        if self.from.x==self.to.x{
            for i in min(self.from.y,self.to.y)..=max(self.from.y,self.to.y){
                vec.push(Point::new(self.from.x,i))
            }
        }else{
            let dist=(self.to.x as i64-self.from.x as i64).abs();
            let dx=(self.to.x as i64-self.from.x as i64)/dist;
            let dy=(self.to.y as i64-self.from.y as i64)/dist;
            for i in 0..=dist{
                vec.push(Point::new((self.from.x as i64+i*dx) as u16,(self.from.y as i64+i*dy) as u16));
            }
        }
        vec
    }
}

#[aoc_generator(day5)]
fn read_input(input: &str) -> Vec<Gift> {
    input.lines().map(|line| {
        Gift::from_coord(scan_fmt!(line,"{},{} -> {},{}",u16,u16,u16,u16).unwrap())
    }).collect()
}

fn day5(input: &[Gift], use_diagonals: bool) -> usize {
    let mut pipes = input.to_vec();
    let mut map: HashMap<Point, u16> = HashMap::new();
    if !use_diagonals {
        pipes.retain(|el| el.from.x == el.to.x || el.from.y == el.to.y);
    }
    for pipe in pipes {
        for p in pipe.points() {
            map.insert(p, map.get(&p).unwrap_or(&0) + 1);
        }
    }
    map.iter().filter(|el| *el.1 > 1).count()
}

//Alternative solution
fn day5_2(input: &[Gift], use_diagonals: bool) -> usize {
    input.iter().filter(|el| use_diagonals || el.from.x == el.to.x || el.from.y == el.to.y)
        .map(|el| el.points())
        .flatten()
        .zip(0..)
        .into_group_map()
        .values()
        .filter(|x| x.len() > 1)
        .count()
}

#[aoc(day5, part1)]
fn day5x1(input: &[Gift]) -> usize {
    day5(input, false)
}

#[aoc(day5, part2)]
fn day5x2(input: &[Gift]) -> usize {
    day5(input, true)
}