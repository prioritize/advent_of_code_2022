use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::Read;

struct DaySix {
    input: VecDeque<char>,
    output: u32,
    deq: VecDeque<char>,
    window_map: HashMap<char, u32>,
    window_size: u32,
}
impl DaySix {
    fn parse(filename: String, window_size: u32) -> Self {
        let mut file = File::open(&filename).expect(&format!("Unable to open {}", filename));
        let mut f_string = String::new();
        file.read_to_string(&mut f_string)
            .expect("unable to parse the file to a string");
        let input: VecDeque<char> = f_string.chars().map(|x| x).collect();
        let mut out = DaySix {
            input,
            output: 0,
            deq: VecDeque::new(),
            window_map: HashMap::new(),
            window_size,
        };
        out.init(window_size);
        out
    }
    fn init(&mut self, window_size: u32) {
        for _ in 0..window_size {
            match self.input.pop_front() {
                None => {}
                Some(v) => {
                    self.update_map(v);
                }
            }
        }
    }
    fn find_first(&mut self) -> u32 {
        let mut idx = self.window_size + 1;
        while !self.step() {
            idx = idx + 1;
        }
        idx
    }
    fn step(&mut self) -> bool {
        let input = self.input.pop_front().unwrap();
        let outgoing = self.deq.pop_front().unwrap();
        match self.window_map.get(&outgoing) {
            None => {
                println!("this shouldn't happen")
            }
            Some(v) => {
                if v == &1 {
                    self.window_map.remove(&outgoing);
                } else {
                    self.window_map.insert(outgoing, v - 1);
                }
            }
        }
        self.update_map(input);
        return self.window_map.len() == self.window_size as usize;
    }
    fn update_map(&mut self, value: char) {
        match self.window_map.get(&value) {
            None => {
                self.window_map.insert(value, 1);
                self.deq.push_back(value);
            }
            Some(j) => {
                self.window_map.insert(value, j + 1);
                self.deq.push_back(value);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_six() {
        let mut seis = DaySix::parse("input/day_6_input.txt".to_string(), 4);
        println!("{}", seis.deq.len());
        assert_ne!(seis.window_map.len(), 4);
        seis.output = seis.find_first();
        println!("{}", seis.output)
    }
    #[test]
    fn test_day_six_test() {
        let mut seis = DaySix::parse("input/day_6_test_input.txt".to_string(), 4);
        println!("{}", seis.deq.len());
        assert_ne!(seis.window_map.len(), 4);
        println!("{}", seis.find_first())
    }
    #[test]
    fn test_day_six_find_message_start() {
        let window_size = 14;
        let mut seis = DaySix::parse("input/day_6_input.txt".to_string(), window_size);
        println!("{}", seis.deq.len());
        assert_ne!(seis.window_map.len(), window_size as usize);
        println!("{}", seis.find_first())
    }
}
