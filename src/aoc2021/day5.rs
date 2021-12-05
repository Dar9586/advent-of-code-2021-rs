use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn execute() {
    assert_eq!(day5x1(), 5147);
    assert_eq!(day5x2(), 16925);
}

#[derive(Default,Debug,Copy, Clone,Eq, PartialEq,Hash)]
struct Point{
    x:u16,
    y:u16
}
#[derive(Debug,Copy, Clone)]
struct Pipe{
    from:Point,
    to:Point
}

impl Point{
    fn new(x:u16,y:u16)->Self{
        Point{x,y}
    }
}

impl Pipe{
    fn from_coord(coords:(u16,u16,u16,u16))->Self{
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

fn read_input()->Vec<Pipe>{
    BufReader::new(File::open("./inputs/2021/day5.txt").unwrap()).lines().map(|line|{
        Pipe::from_coord(scan_fmt!(&line.unwrap(),"{},{} -> {},{}",u16,u16,u16,u16).unwrap())
    }).collect()
}

fn day5(use_diagonals:bool) -> usize {
    let mut pipes=read_input();
    let mut map:HashMap<Point,u16>=HashMap::new();
    if !use_diagonals {
        pipes.retain(|el| el.from.x == el.to.x || el.from.y == el.to.y);
    }
    for pipe in pipes{
        for p in pipe.points(){
            map.insert(p,map.get(&p).unwrap_or(&0)+1);
        }
    }
    map.iter().filter(|el|*el.1>1).count()
}

fn day5x1() -> usize {
   day5(false)
}

fn day5x2() -> usize {
    day5(true)
}