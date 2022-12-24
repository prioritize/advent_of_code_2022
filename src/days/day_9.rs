use nom::character::complete::{alpha1, char, digit1};
use nom::combinator::map;
use nom::sequence::{separated_pair};
use nom::IResult;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;
#[derive(Copy, Clone, Eq, Hash)]
struct Position {
    row: i32,
    col: i32,
}
impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}
impl Position {
    fn new() -> Self {
        Position {
            row: 0,
            col: 0,
        }
    }
}
impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "row: {}, col: {}", self.row, self.col)
    }
}
fn distance(a: Position, b: Position) -> i32 {
    let x = (a.row - b.row).pow(2);
    let y = (a.col - b.col).pow(2);
    ((x + y) as f64).sqrt() as i32
}
fn follow(head: Position, tail: Position) -> Position {
    if head.row == tail.row && head.col == tail.col {
        return tail;
    } else if head.row != tail.row && head.col != tail.col {
        if distance(head, tail) <= 1 {
            return tail;
        }
        return move_diagonal(head, tail);
    } else if head.row != tail.row || head.col != tail.col {
        if head.row != tail.row {
            return move_row(head, tail);
        } else if head.col != tail.col {
            return move_col(head, tail);
        }
    }
    tail
}
fn move_diagonal(head: Position, tail:Position) -> Position {
    let mut out = Position::new();
    match head.col - tail.col {
        0 => {panic!("This should have been filtered out by the distance equation")}
        i32::MIN..=-1 => out.col = tail.col - 1,
        1..=i32::MAX => out.col = tail.col + 1,
    }
    match head.row - tail.row {
        0 => {panic!("This should have been filtered out by the distance equation")}
        i32::MIN..=-1 => out.row = tail.row - 1,
        1..=i32::MAX => out.row = tail.row + 1,
    }
    out
}
fn move_row(head: Position, tail: Position) -> Position {
    let mut out = Position::new();
    match head.row - tail.row {
        -1..=1 => {out.row = tail.row}
        i32::MIN..=-2 => out.row = tail.row - 1,
        2..=i32::MAX => out.row = tail.row + 1,
    }
    out.col = tail.col;
    out
}
fn move_col(head: Position, tail: Position) -> Position {
    let mut out = Position::new();
    match head.col - tail.col {
        -1..=1 => {out.col = tail.col}
        i32::MIN..=-2 => out.col = tail.col - 1,
        2..=i32::MAX => out.col = tail.col + 1,
    }
    out.row = tail.row;
    out
}
struct Knot {
    pos: Position,
    visited: HashMap<(i32, i32), bool>,
}
impl Knot {
    fn new() -> Self {
        let mut hm = HashMap::new();
        hm.insert((0, 0), true);
       Knot {
           pos: Position::new(),
           visited: hm,
       }
    }
    fn position(&self) -> Position {
        self.pos
    }
}
impl Display for Knot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Knot( {} )", self.pos)
    }
}
struct Rope {
    knots: Vec<Knot>,
    length: usize,
}
impl Rope {
    fn new(length: usize) -> Self {
        let mut knots: Vec<Knot> = Vec::new();
        for _ in 0..length {
            knots.push(Knot::new());
        }
        Rope {
            knots,
            length
        }

    }
    fn head(&mut self) -> &mut Knot{
        &mut self.knots[0]
    }
    fn traverse(&mut self, movements: &Vec<Movement>) {
        movements.iter().for_each(|x| {
            self.instruction(x);
        });
    }
    fn instruction(&mut self, movement: &Movement) {
        for _ in 0..movement.magnitude {
            self.step(&movement.dir);
            self.move_tail();
        }
    }
    fn step(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => {
                self.head().pos.row = self.head().pos.row - 1;
            }
            Direction::Down => {
                self.head().pos.row = self.head().pos.row + 1;
            }
            Direction::Left => {
                self.head().pos.col = self.head().pos.col - 1;
            }
            Direction::Right => {
                self.head().pos.col = self.head().pos.col + 1;
            }
        }
    }
    fn move_tail(&mut self) {
        for idx in 1..self.knots.len() {
            let head = self.knots.get(idx-1 as usize).unwrap().position();
            let tail = self.knots.get(idx).unwrap().position();
            let tail_pos = follow(head, tail);
            self.knots.get_mut(idx).unwrap().pos = tail_pos;
            self.knots.get_mut(idx).unwrap().visited.insert((tail_pos.row, tail_pos.col), true);
        }
    }
}
struct Movement {
    dir: Direction,
    magnitude: u32,
}
impl Display for Movement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Direction: {}, Magnitude: {}", self.dir, self.magnitude)
    }
}
impl Movement {
    fn parse(input: &str) -> IResult<&str, Self> {
        let two_words_parser = separated_pair(alpha1, char(' '), digit1);
        let mut person_parser = map(two_words_parser, |(dir, magnitude)| {
            let dir = match dir {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Got a value we didn't expect"),
            };
            Self {
                dir,
                magnitude: magnitude.parse::<u32>().unwrap(),
            }
        });
        person_parser(input)
    }
}
enum Direction {
    Right,
    Left,
    Up,
    Down,
}
impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Right => {
                write!(f, "Right")
            }
            Direction::Left => {
                write!(f, "Left")
            }
            Direction::Up => {
                write!(f, "Up")
            }
            Direction::Down => {
                write!(f, "Down")
            }
        }
    }
}
fn parse_input(filename: String) -> Vec<Movement> {
    let mut file = File::open(&filename).unwrap();
    let mut string_buf = String::new();
    let _ = file.read_to_string(&mut string_buf).unwrap();
    string_buf
        .lines()
        .map(|x| Movement::parse(x).unwrap().1)
        .collect()
}

#[cfg(test)]
mod tests {
use super::*;
    #[test]
    fn test_part_1() {
        let movements = parse_input("input/day_9_input.txt".to_string());
        println!("len of movements is {}", movements.len());
        assert_ne!(0, movements.len());
        let mut rope = Rope::new(2);
        rope.traverse(&movements);
        println!("{}", rope.knots[1].visited.len());
    }
    #[test]
    fn test_part_2() {
        let movements = parse_input("input/day_9_input.txt".to_string());
        println!("len of movements is {}", movements.len());
        assert_ne!(0, movements.len());
        let mut rope = Rope::new(10);
        rope.traverse(&movements);
        println!("{}", rope.knots[rope.length-1].visited.len())
    }
    #[test]
    fn test_part_2_example() {
        let movements = parse_input("input/day_9_test_input_2.txt".to_string());
        println!("len of movements is {}", movements.len());
        assert_ne!(0, movements.len());
        let mut rope = Rope::new(10);
        rope.traverse(&movements);
        println!("{}", rope.knots[rope.length-1].visited.len())
    }
}
