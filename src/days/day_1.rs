use fs::File;
use std::collections::BinaryHeap;
use std::fs;
use std::io::Read;

struct DayOne {
    output: Vec<Vec<u32>>,
}
impl DayOne {
    fn parse(filename: String) -> u32 {
        let mut file = File::open(&filename).expect(&format!("Unable to open {}", filename));
        let mut f_string = String::new();
        file.read_to_string(&mut f_string)
            .expect(&format!("Unable to read the file into a string"));
        let mut running_total = 0;
        let mut hungriest = 0;
        let mut heap = BinaryHeap::new();
        for line in f_string.lines() {
            if line == "" {
                if running_total > hungriest {
                    hungriest = running_total;
                }
                heap.push(running_total);
                running_total = 0;
                continue;
            }
            running_total = running_total
                + line
                    .parse::<u32>()
                    .expect("Unable to create a u32 from this line")
        }
        println!(
            "The amount of calories the hungriest elf is carrying is {}",
            hungriest
        );
        println!(
            "the amount of calories the top three elves are carrying is {}",
            heap.pop().unwrap() + heap.pop().unwrap() + heap.pop().unwrap()
        );
        hungriest
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_one() {
        println!("{}", DayOne::parse("input/day_1_input.txt".to_string()));
    }
}
