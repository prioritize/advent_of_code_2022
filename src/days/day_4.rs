use std::fs::File;
use std::io::Read;
use regex;

struct DayFour {
    output: u32,
}
#[derive(Debug)]
struct Range {
    start: u32,
    end: u32,
    range: u32,
}
impl PartialEq for Range {
    fn eq(&self, other: &Self) -> bool {
        self.start() == other.start() && self.end() == other.end()
    }
}
impl Range {
    fn range(&self) -> u32 {
        self.range
    }
    fn start(&self) -> u32 {
        self.start
    }
    fn end(&self) -> u32 {
        self.end
    }
    fn new(start: u32, end: u32) -> Self {
        let mut r = Range{
            start,
            end,
            range: 0,
        };
        r.range = r.end() - r.start();
        r
    }
    fn contained(&self, other: &Range) -> bool {
       return match other.start() <= self.start() {
           true => {
               return match other.end() >= self.end() {
                   true => {
                       true
                   },
                   false => {
                       false
                   }
               }
           }
           false => {false}
       }
    }
    fn early_overlap(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.start
    }
    fn late_overlap(&self, other: &Range) -> bool {
        self.start >= other.start && self.start <= other.end
    }
    fn overlap(&self, other: &Range) -> bool {
        self.early_overlap(other) || self.late_overlap(&other)
    }
}
impl DayFour {
    fn parse(filename: String) -> Vec<Vec<Range>> {
        let mut file = File::open(&filename).expect(&format!("Unable to open {}", filename));
        let mut f_string = String::new();
        file.read_to_string(&mut f_string);
        let re = regex::Regex::new(r"((([0-9]{1,6})-([0-9]{1,6})),(([0-9]{1,6})-([0-9]{1,6})))\n").unwrap();
        re.captures_iter(&f_string).map(|x| {
            vec![Range::new(x[3].parse().unwrap(), x[4].parse().unwrap()), Range::new(x[6].parse().unwrap(), x[7].parse().unwrap())]
            }
        ).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_four() {
        let ranges = DayFour::parse("input/day_4_input.txt".to_string());

        let total_contained = ranges.iter().map(|x| {
            x[0].contained(&x[1]) || x[1].contained(&x[0])
        }).filter(|b| b == &true).fold(0, |acc, v| acc + 1);
        println!("total contained: {}", total_contained);
        let total_overlap = ranges.iter().map(|x| {
            x[0].contained(&x[1]) || x[1].contained(&x[0]) || x[0].overlap(&x[1])
        }).filter(|b| b == &true).fold(0, |acc, v| acc + 1);
        println!("total overlap: {}", total_overlap);
    }

}