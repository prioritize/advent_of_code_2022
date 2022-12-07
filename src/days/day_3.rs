use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::iter::zip;

fn make_priority_map() -> HashMap<char, u32> {
    let mut hm = HashMap::new();
    let alpha = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut increment = 1;
    alpha.chars().for_each(|x| {
        hm.insert(x, increment);
        increment = increment + 1;
    });
    hm
}
struct DayThree {
    out: u32,
}
impl DayThree {
    fn parse(filename: String) {
        let mut file = File::open(&filename).expect(&format!("Unable to open {}", filename));
        let mut f_string = String::new();
        file.read_to_string(&mut f_string);
        let duplicates: Vec<char> = f_string
            .lines()
            .map(|x| {
                let center = x.len() / 2;
                let strang = x.to_string();
                let begin = strang[0..center].to_string();
                let end = strang[center..].to_string();
                find_char(begin, end)
            })
            .collect();
        let p = make_priority_map();
        let priority_sum = duplicates
            .iter()
            .map(|x| p.get(x).unwrap())
            .fold(0, |acc, v| acc + v);
        println!("The sum of the priorities is {}", priority_sum);
        let maps: Vec<HashMap<char, u32>> = f_string
            .lines()
            .map(|x| {
                let mut hm = HashMap::new();
                x.chars().for_each(|v| match hm.get(&v) {
                    None => {
                        hm.insert(v, 1);
                    }
                    Some(count) => {
                        hm.insert(v, count + 1);
                    }
                });
                hm
            })
            .collect();
        let groups: Vec<&[HashMap<char, u32>]> = maps.chunks(3).collect();
        let chars: Vec<char> = groups
            .iter()
            .map(|x| find_shared_character(&x[0], &x[1], &x[2]))
            .collect();
        let badge_total = chars
            .iter()
            .map(|x| p.get(x).unwrap())
            .fold(0, |acc, v| acc + v);
        println!("the priority total of the badges is {}", badge_total);
    }
}
fn find_char(begin: String, end: String) -> char {
    let mut hm: HashMap<char, u32> = HashMap::new();
    let mut out: char = '9';

    for (l, r) in begin.chars().zip(end.chars()) {
        match hm.insert(l, 0) {
            None => {}
            Some(v) => {
                if v != 0 {
                    return l;
                }
            }
        }
        match hm.insert(r, 1) {
            None => {}
            Some(v) => {
                if v != 1 {
                    return r;
                }
            }
        };
    }
    '!'
}
fn find_shared_character(
    first: &HashMap<char, u32>,
    second: &HashMap<char, u32>,
    third: &HashMap<char, u32>,
) -> char {
    let answer: Vec<Option<&char>> = first
        .iter()
        .map(|(k, v)| match second.get(k) {
            Some(_) => match third.get(k) {
                Some(_) => Some(k),
                None => None,
            },
            None => None,
        })
        .filter(|x| match x {
            None => false,
            Some(_) => true,
        })
        .collect();
    answer[0].unwrap().clone()
}

#[cfg(test)]
mod tests {
    use crate::days::day_3::{make_priority_map, DayThree};

    #[test]
    fn parse_day_three() {
        println!("----------- DAY 3 -------------");
        DayThree::parse("input/day_3_input.txt".to_string());
    }
    #[test]
    fn test_priority_a() {
        let p = make_priority_map();
        assert_eq!(p.get(&'a').unwrap().clone(), 1);
    }
    #[test]
    fn test_priority_A() {
        let p = make_priority_map();
        assert_eq!(p.get(&'A').unwrap().clone(), 27);
    }
    #[test]
    fn parse_day_three_test() {
        DayThree::parse("input/day_3_test_input.txt".to_string());
    }
}
