use itertools::enumerate;
use std::fs::File;
use std::io::Read;

struct DayEight {}
impl DayEight {
    fn open_input_file(filename: String) -> Vec<Vec<u32>> {
        let mut file = File::open(&filename).unwrap();
        let mut string_buf = String::new();
        let f_string = file.read_to_string(&mut string_buf).unwrap();
        // let mut tree_height = Vec::new();
        let tree_height = string_buf
            .lines()
            .map(|x| x.chars().map(|v| v.to_digit(10).unwrap()).collect())
            .collect();
        tree_height
    }
}
fn can_see_tree(height: &u32, highest: &u32) -> bool {
    height > highest
}
fn horizontal(tree_height: &Vec<Vec<u32>>, visible: &mut Vec<Vec<bool>>) {
    for (r_idx, r) in enumerate(tree_height) {
        let mut highest_r: Option<&u32> = None;
        let mut highest_l: Option<&u32> = None;
        for (c_idx, c) in enumerate(r).rev() {
            match highest_r {
                None => {
                    highest_r = Some(c);
                    visible[r_idx][c_idx] = true;
                }
                Some(h) => {
                    if can_see_tree(c, h) {
                        visible[r_idx][c_idx] = true;
                        highest_r = Some(c);
                    }
                }
            }
        }
        for (c_idx, c) in enumerate(r) {
            match highest_l {
                None => {
                    highest_l = Some(c);
                    visible[r_idx][c_idx] = true;
                }
                Some(h) => {
                    if can_see_tree(c, h) {
                        visible[r_idx][c_idx] = true;
                        highest_l = Some(c);
                    }
                }
            }
        }
    }
}
fn vertical(trees: &Vec<Vec<u32>>, visible: &mut Vec<Vec<bool>>) {
    for c_idx in 0..trees[0].len() {
        let mut highest: Option<&u32> = None;
        for r_idx in 0..trees.len() {
            match highest {
                None => {
                    highest = Some(&trees[r_idx][c_idx]);
                    visible[r_idx][c_idx] = true;
                }
                Some(h) => {
                    if can_see_tree(&trees[r_idx][c_idx], h) {
                        visible[r_idx][c_idx] = true;
                        highest = Some(&trees[r_idx][c_idx]);
                    }
                }
            }
        }
        highest = None;
        for r_idx in (0..trees.len()).rev() {
            match highest {
                None => {
                    highest = Some(&trees[r_idx][c_idx]);
                    visible[r_idx][c_idx] = true;
                }
                Some(h) => {
                    if can_see_tree(&trees[r_idx][c_idx], h) {
                        visible[r_idx][c_idx] = true;
                        highest = Some(&trees[r_idx][c_idx]);
                    }
                }
            }
        }
    }
}
fn from_down() {}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    #[test]
    fn test_open_input() {
        let trees = DayEight::open_input_file("input/day_8_input.txt".to_string());
        let mut visible: Vec<Vec<bool>> = trees
            .iter()
            .map(|x| x.iter().map(|v| false).collect())
            .collect();
        horizontal(&trees, &mut visible);
        vertical(&trees, &mut visible);
        let true_lines: Vec<Vec<bool>> = visible
            .iter()
            .map(|b| b.iter().filter(|x| x == &&true).cloned().collect())
            .collect();
        let visible_count: Vec<u32> = true_lines
            .iter()
            .map(|i| i.iter().fold(0, |acc, _| acc + 1))
            .collect();
        let out = visible_count.iter().fold(0, |acc, v| acc + v);
        println!("Total visible trees: {}", out);
    }
    #[test]
    fn test_open_test_input() {
        let trees = DayEight::open_input_file("input/day_8_test_input.txt".to_string());
        let mut visible: Vec<Vec<bool>> = trees
            .iter()
            .map(|x| x.iter().map(|v| false).collect())
            .collect();
        horizontal(&trees, &mut visible);
        vertical(&trees, &mut visible);
        let true_lines: Vec<Vec<bool>> = visible
            .iter()
            .map(|b| b.iter().filter(|x| x == &&true).cloned().collect())
            .collect();
        let visible_count: Vec<u32> = true_lines
            .iter()
            .map(|i| i.iter().fold(0, |acc, _| acc + 1))
            .collect();
        let out = visible_count.iter().fold(0, |acc, v| acc + v);
        println!("Total visible trees: {}", out);
    }
}
