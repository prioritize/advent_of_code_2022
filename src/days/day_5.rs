use regex::{Captures, Regex};
use std::fs::File;
use std::io::Read;

struct DayFive {
    out: String,
}

struct Instruction {
    count: usize,
    source: usize,
    dest: usize,
}
impl Instruction {
    pub fn new(cap: Captures) -> Self {
        Instruction {
            count: cap[1].to_string().parse::<usize>().unwrap(),
            source: cap[2].to_string().parse::<usize>().unwrap() - 1,
            dest: cap[3].to_string().parse::<usize>().unwrap() - 1,
        }
    }
    fn execute(&self, crates: &mut Vec<Vec<String>>) {
        for _ in 0..self.count {
            let current = crates[self.source].pop();
            match current {
                None => {}
                Some(v) => crates[self.dest].push(v),
            }
        }
    }
    fn execute_9001(&self, crates: &mut Vec<Vec<String>>) {
        let len = crates[self.source].len();
        let b_range = len.checked_sub(self.count);
        let b_range = match b_range {
            None => 0,
            Some(v) => v,
        };
        if len - b_range <= 0 {
            println!("No boxes to move");
            return;
        };
        let mut sub = Vec::from_iter(crates[self.source][b_range..len].iter().cloned());
        crates[self.source].drain(b_range..len);
        crates[self.dest].append(&mut sub);
    }
}

impl DayFive {
    fn parse(filename: String) -> String {
        let mut file = File::open(&filename).expect(&format!("Unable to open {}", filename));
        let mut f_string = String::new();
        file.read_to_string(&mut f_string)
            .expect("unable to parse the file to a string");

        let stack_count_re = Regex::new(r"\s([0-9]{1,3})\s{2}").unwrap();
        let count = stack_count_re
            .captures_iter(&f_string)
            .fold(0, |acc, _| acc + 1);

        let mut stacks: Vec<Vec<String>> = Vec::new();
        for _ in 0..count {
            stacks.push(Vec::new())
        }

        let re = Regex::new(r"([ \n]{4}|\[([A-Z])\][ \n])([ \n]{4}|\[([A-Z])\][ \n])([ \n]{4}|\[([A-Z])\][ \n])([ \n]{4}|\[([A-Z])\][ \n])([ \n]{4}|\[([A-Z])\][ \n])([ \n]{4}|\[([A-Z])\][ \n])([ \n]{4}|\[([A-Z])\][ \n])([ \n]{4}|\[([A-Z])\][ \n])([ \n]{4}|\[([A-Z])\][ \n])").unwrap();
        re.captures_iter(&f_string).for_each(|x| {
            let mut idx = 2;
            while idx < x.len() {
                match x.get(idx) {
                    None => {}
                    Some(i) => {
                        stacks[(idx / 2) - 1].push(i.as_str().to_string());
                    }
                }
                idx = idx + 2;
            }
        });

        let instructions_re =
            Regex::new(r"move ([0-9]{1,4}) from ([0-9]{1,4}) to ([0-9]{1,4})\n").unwrap();
        let instructions: Vec<Instruction> = instructions_re
            .captures_iter(&f_string)
            .map(|x| Instruction::new(x))
            .collect();

        let mut rev_stacks: Vec<Vec<String>> = stacks
            .into_iter()
            .map(|x| {
                let mut rev = x.clone();
                rev.reverse();
                return rev;
            })
            .collect();
        let mut stacks_on_stacks = rev_stacks.clone();
        for stack in &rev_stacks {
            println!("{:?}", stack);
        }
        instructions.iter().for_each(|x| x.execute(&mut rev_stacks));
        instructions
            .iter()
            .for_each(|x| x.execute_9001(&mut stacks_on_stacks));
        for stack in &rev_stacks {
            let len = stack.len() - 1;
            print!("{}", stack[len])
        }
        println!();
        for stack in &stacks_on_stacks {
            let len = stack.len() - 1;
            print!("{}", stack[len])
        }
        println!();
        "out".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_five() {
        DayFive::parse("input/day_5_input.txt".to_string());
    }
}
