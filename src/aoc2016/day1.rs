use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::ops;

use text_io::try_read;

/*The input must be changed from having comma space to only comma and adding a trailing comma*/
pub fn execute() {
    assert_eq!(day1x1(), 161);
    assert_eq!(day1x2(), 110);
}

enum Direction { Up, Right, Down, Left }

#[derive(Eq, PartialEq, Hash, Copy, Clone, Default)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }
}

impl ops::Mul<i64> for Point {
    type Output = Point;

    fn mul(self, rhs: i64) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl ops::AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Direction {
    fn dir(&self) -> Point {
        match self {
            Direction::Up => Point::new(0, 1),
            Direction::Down => Point::new(0, -1),
            Direction::Left => Point::new(-1, 0),
            Direction::Right => Point::new(1, 0)
        }
    }
    fn left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
    fn right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

fn day1x1() -> i64 {
    let mut file = File::open("./inputs/2016/day1.txt").expect("Invalid input file").bytes().map(|ch| ch.unwrap());
    let mut pos = Point::default();
    let mut direction = Direction::Up;
    while let Some(dir) = file.next() {
        let amount: i64 = try_read!("{},",file).unwrap();
        direction = if dir == b'L' { direction.left() } else { direction.right() };
        pos += direction.dir() * amount
    }
    return pos.x.abs() + pos.y.abs();
}

fn day1x2() -> i64 {
    let mut file = File::open("./inputs/2016/day1.txt").expect("Invalid input file").bytes().map(|ch| ch.unwrap());
    let mut pos = Point::default();
    let mut direction = Direction::Up;
    let mut set: HashSet<Point> = HashSet::new();
    while let Some(dir) = file.next() {
        let amount: i64 = try_read!("{},",file).unwrap();
        direction = if dir == b'L' { direction.left() } else { direction.right() };
        for _ in 0..amount {
            pos += direction.dir();
            if !set.insert(pos) {
                return pos.x.abs() + pos.y.abs();
            }
        }
    }
    unreachable!();
}