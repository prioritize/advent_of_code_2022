use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;
use std::ops::Add;
use itertools::{enumerate, Position};
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1};
use nom::combinator::opt;
use nom::{IResult, ParseTo};

struct CRT {
    sprite: [i32; 3],
    screen: Vec<Vec<char>>,
    line: Vec<char>,
}
impl CRT {
    fn new() -> Self {
        CRT {
            sprite: [-1, 0, 1],
            screen: Vec::new(),
            line: Vec::new(),
        }
    }
    fn increment_sprite(&mut self) {
        for location in  0..self.sprite.len() {
            if let Some(elem) = self.sprite.get_mut(location) {
                *elem = *elem + 1;
            }
        }
    }
    fn draw(&mut self, x: i32) {
        let center = self.sprite[1];
        for pixel in 0..self.sprite.len() {
            if x == *self.sprite.get(pixel).unwrap() {
                self.line.push('#');
                return
            }
        }
        self.line.push('.')
    }
    fn print(&self) {
        for (idx, line) in enumerate(self.screen.clone()) {
            println!("{:?}", line);
        }
        println!();
    }
}
struct CPU {
    register: i32,
    history: Vec<Cycle>,
}
struct Cycle {
    before: i32,
    during: i32,
    after: i32,
}
impl Cycle {
    fn new() -> Self {
        Cycle {
            before: 0,
            during: 0,
            after: 0,
        }
    }
}
enum Instruction {
    Noop,
    Addx { amount: i32 },
}
impl Display for Instruction{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Noop => {
                write!(f, "No-Op")
            }
            Instruction::Addx { amount } => {
                write!(f, "Add: {}", amount)
            }
        }
    }
}
fn parse_noop(i: &str) -> IResult<&str, &str> {
    tag("noop")(i)
}
fn parse_addx(i: &str) -> IResult<&str, &str> {
    tag("addx")(i)
}
impl Instruction {
    fn parse_input(filename: &str) -> Vec<Instruction> {
        let mut file = File::open(filename).unwrap();
        let mut string_buf = String::new();
        let _ = file.read_to_string(&mut string_buf).unwrap();
        string_buf.lines().map(|x| {
            match parse_addx(x) {
               Ok(ans) => {
                   match ans.0[1..].parse::<i32>() {
                       Ok(number) => {
                           Instruction::Addx {amount: number}
                       }
                       Err(_) => {
                           println!("{}", ans.0);
                           panic!("not parsing correctly");
                       }
                   }
               },
                Err(_) => {
                    Instruction::Noop
                }
            }
        }).collect()
    }
    fn pre(&self, cycle: &mut Cycle, register: i32) {
        match self {
            Instruction::Noop => {
                cycle.before = register
            }
            Instruction::Addx { .. } => {
                cycle.before = register;
            }
        }
    // Set the value of the Cycle.before here
    }
    fn during(&self, cycle: &mut Cycle, register: i32, crt: &mut CRT) {
        match self {
            Instruction::Noop => {
                cycle.during = register;
                crt.draw(register);
            }
            Instruction::Addx { .. } => {
                cycle.during = register;
                crt.draw(register);
            }
        }
    //     The draw happens here
    }
    fn post(&self, cycle: &mut Cycle, register: i32) {
        match self {
            Instruction::Noop => {
                cycle.after = register;
            }
            Instruction::Addx { amount } => {
                cycle.after = register + amount;
            }
        }
    }
}
impl CPU {
    fn execute(&mut self, cmd: &Instruction, crt: &mut CRT) {
        let mut cycle = Cycle::new();
        match cmd {
            Instruction::Noop => {
                cmd.pre(&mut cycle, self.register);
                cmd.during(&mut cycle, self.register, crt);
                cmd.post(&mut cycle, self.register);
                self.history.push(cycle);
                crt.increment_sprite();
                if crt.line.len() == 40 {
                    crt.screen.push(crt.line.clone());
                    crt.line.clear();
                    crt.sprite = [-1, 0, 1];
                }
            }
            Instruction::Addx { amount } => {
                self.execute(&Instruction::Noop, crt);
                cmd.pre(&mut cycle, self.register);
                cmd.during(&mut cycle, self.register, crt);
                cmd.post(&mut cycle, self.register);
                self.register = cycle.after;
                self.history.push(cycle);
                crt.increment_sprite();
                if crt.line.len() == 40 {
                    crt.screen.push(crt.line.clone());
                    crt.line.clear();
                    crt.sprite = [-1, 0, 1];
                }
            }
        }
    }
    fn new() -> Self {
        CPU {
            register: 1,
            history: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_test() {
        println!("Start of Test");
        let out = Instruction::parse_input("input/day_10_test_input.txt");
        assert_ne!(0, out.len());
        let mut cpu = CPU::new();
        let mut crt = CRT::new();
        out.iter().for_each(|x| {
            cpu.execute(x, &mut crt);
        });
        let values = vec![19, 59, 99, 139, 179, 219];
        let mut total = 0;
        values.iter().for_each(|x| {
            total = total + (x + 1) * cpu.history.get(*x as usize).unwrap().during;
        });
        println!("The length of the CRT values is: {}", crt.screen.len());
        crt.print();
        println!("total: {}", total);
        println!("End of Test");
    }

    #[test]
    fn test_part_1() {
        println!("Start of Input Evaluation");
        let out = Instruction::parse_input("input/day_10_input.txt");
        assert_ne!(0, out.len());
        let mut cpu = CPU::new();
        let mut crt = CRT::new();
        out.iter().for_each(|x| {
            cpu.execute(x, &mut crt);
        });
        let values = vec![19, 59, 99, 139, 179, 219];
        let mut total = 0;
        values.iter().for_each(|x| {
            total = total + (x + 1) * cpu.history.get(*x as usize).unwrap().during;
        });
        crt.print();
        println!("total: {}", total);
        println!("End of Input Evaluation")
    }
    #[test]
    fn test_sprite_increment() {
        println!("Start of Sprite Increment Test");
        let mut system = CRT::new();
        let init = [-1, 0, 1];
        assert_eq!(init, system.sprite);
        system.increment_sprite();
        let init = [0, 1, 2];
        assert_eq!(init, system.sprite);
        println!("End of Sprite Increment Test")
    }
}
