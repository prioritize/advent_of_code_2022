use itertools::enumerate;
use std::fs::File;
use std::io::Read;
struct Tree {
    height: u32,
    row: usize,
    col: usize,
}
struct Forest {
    trees: Vec<Vec<Tree>>,
}

impl Tree {
    pub fn look(
        location: (usize, usize),
        height: u32,
        direction: &Direction,
        forest: &Forest,
    ) -> Option<bool> {
        match forest.next((location.0, location.1), direction) {
            None => None,
            Some(tree) => Some(height > tree.height),
        }
    }
    fn measure(&self, direction: &Direction, forest: &Forest) -> u32 {
        let mut loc = (self.row, self.col);
        let mut acc = 0;
        loop {
            match Self::look(loc, self.height, direction, forest) {
                None => break,
                Some(v) => match v {
                    true => {
                        acc = acc + 1;
                        loc = neighbor(loc, direction);
                    }
                    false => {
                        acc = acc + 1;
                        break;
                    }
                },
            }
        }
        acc
    }
}

impl Forest {
    fn new(filename: String) -> Self {
        let mut file = File::open(&filename).unwrap();
        let mut string_buf = String::new();
        let f_string = file.read_to_string(&mut string_buf).unwrap();
        let mut trees: Vec<Vec<Tree>> = Vec::new();
        for (r, line) in enumerate(string_buf.lines()) {
            let mut col = 0;
            trees.push(
                line.chars()
                    .map(|x| {
                        let out = Tree {
                            height: x.to_digit(10).unwrap(),
                            row: r,
                            col,
                        };
                        col = col + 1;
                        out
                    })
                    .collect(),
            )
        }
        Forest { trees }
    }
    fn next(&self, current: (usize, usize), direction: &Direction) -> Option<&Tree> {
        let neighbor = neighbor(current, &direction);
        return match self.trees.get(neighbor.0) {
            None => None,
            Some(v) => v.get(neighbor.1),
        };
    }
    fn best_view(&self) -> u32 {
        let views: Vec<Vec<u32>> = self
            .trees
            .iter()
            .map(|r| {
                r.iter()
                    .map(|c| {
                        c.measure(&Direction::North, &self)
                            * c.measure(&Direction::South, &self)
                            * c.measure(&Direction::East, &self)
                            * c.measure(&Direction::West, &self)
                    })
                    .collect()
            })
            .collect();
        *views
            .iter()
            .map(|v| *v.iter().max().unwrap())
            .collect::<Vec<u32>>()
            .iter()
            .max()
            .unwrap()
        // *maxes.iter().max().unwrap()
    }
}
fn neighbor(loc: (usize, usize), direction: &Direction) -> (usize, usize) {
    match direction {
        Direction::North => (loc.0 - 1, loc.1),
        Direction::South => (loc.0 + 1, loc.1),
        Direction::East => (loc.0, loc.1 + 1),
        Direction::West => (loc.0, loc.1 - 1),
    }
}
struct DayEight {}
impl DayEight {
    fn open_input_file(filename: String) -> Vec<Vec<u32>> {
        let mut file = File::open(&filename).unwrap();
        let mut string_buf = String::new();
        let f_string = file.read_to_string(&mut string_buf).unwrap();
        // let mut tree_height = Vec::new();
        let mut trees: Vec<Vec<Tree>> = Vec::new();
        for (r, line) in enumerate(string_buf.lines()) {
            let mut col = 0;
            trees.push(
                line.chars()
                    .map(|x| {
                        let out = Tree {
                            height: x.to_digit(10).unwrap(),
                            row: r,
                            col,
                        };
                        col = col + 1;
                        out
                    })
                    .collect(),
            )
        }
        let tree_height = string_buf
            .lines()
            .map(|x| x.chars().map(|v| v.to_digit(10).unwrap()).collect())
            .collect();
        tree_height
    }
}
enum Direction {
    North,
    South,
    East,
    West,
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
    #[test]
    fn part_two() {
        let forest = Forest::new("input/day_8_input.txt".to_string());
        println!("{}", forest.best_view());
    }
    #[test]
    fn part_two_test() {
        let forest = Forest::new("input/day_8_test_input.txt".to_string());
        println!("{}", forest.best_view());
    }
}
