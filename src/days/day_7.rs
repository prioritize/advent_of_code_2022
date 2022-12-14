use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;
use regex::{Captures, Match, Regex};

struct Directory {
    files: HashMap<String, u32>,
    directories: HashMap<String, Directory>,
    parent: Option<String>,
    name: String,
    size_contained: u32,
}
impl Directory {
    fn add_dir(&mut self, name: String) {
       match self.directories.get(&name) {
           None => {
               self.directories.insert(name.clone(), Directory::new(name.clone()));
           }
           Some(_) => {
               println!("There was already a directory named {} in the directory named {}", name, self.name);
           }
       };
    }
    fn new(name: String) -> Self {
        Directory {
            files: HashMap::new(),
            directories: HashMap::new(),
            parent: None,
            name,
            size_contained: 0
        }
    }
    // fn change_directory(&mut self, name: String) -> &mut Directory {
    //     self.directories.get(&name).unwrap()
    // }
    fn add_file(&mut self, name: String, size: u32) {
        match self.files.get(&name) {
            None => {
                self.files.insert(name.clone(), size);
                self.size_contained = self.size_contained + size;
            }
            Some(_) => {
                println!("There was already a file named {} in the directory named {}", name, self.name);
            }
        };
    }
}
struct DaySeven {
    out: Directory
}
impl DaySeven {
    fn parse(filename: String) {
        let mut file = File::open(&filename).expect(&format!("Unable to open {}", filename));
        let mut f_string = String::new();
        let mut head = Directory::new("/".to_string());
        let mut current = &mut head;
        file.read_to_string(&mut f_string).expect(&format!("Unable to open {}", &filename));
        let re = Regex::new(r"/(\$ cd ([\w\/.]*)\n)|(\$ ls\n)|(dir ([\w.]{1,20})\n)|(([\d]{1,20}) ([\w.]{1,20})\n)").unwrap();
        let mut iter = f_string.lines();
        while let Some(line) = iter.next() {
            let cap = re.captures(line).unwrap();
            match cap.get(2) {
                None => {}
                Some(v) => {
                    if v.as_str().to_string() == "/".to_string() {
                        current = &mut head;
                    } else {
                        // current = current.change_directory(v.as_str().to_string())
                    }
                }
            }
            match cap.get(3) {
            //     We need to iterate here until we get to another  cd or ls
                None => {},
                Some(_) => {
                    while let Some(entry) = iter.next() {
                        let line_cap = re.captures(entry).unwrap();
                        match line_cap.get(5) {
                            None => {},
                            Some(dir) => {
                                match current.directories.get(dir.as_str()) {
                                    None => {current.add_dir(dir.as_str().to_string())}
                                    Some(_) => {}
                                }
                            }
                        }
                    }
                }

            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn test_day_7() {
        DaySeven::parse("input/day_7_input.txt".to_string());
        assert_eq!(0, 0)
    }
}