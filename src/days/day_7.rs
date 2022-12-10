use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;
use regex::{Captures, Match, Regex};

pub trait AoC: Display {
    fn print(&self){
        println!("This is super functional")
    }
}
enum AoCObject{
    File {
        name: String,
        size: u32,
    },
    Directory {
        files: HashMap::<String, AoCObject>,
        dirs: HashMap::<String, AoCObject>,
        name: String,
        parent: Option<String>,
    },
    LS,
    CD {
        next_dir: String,
    }

}
impl AoC for AoCObject {}
impl Display for AoCObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            AoCObject::File{ name, size } => {
                write!(f, "AoCFile( Name: {}, Size: {})", name, size)
            }
            AoCObject::Directory { name,  .. } => {
                write!(f, "AoCDir( Name: {})", name)
            }
            AoCObject::LS => {
                write!(f, "LS")
            }
            AoCObject::CD { next_dir} => {
                write!(f, "CD ( next: {} )", next_dir)
            }
        }
    }
}
impl AoCObject {
    pub fn new_file(name: String, size: u32) -> Self {
        AoCObject::File{
            name,
            size
        }
    }
    pub fn new_dir(name: String, parent: Option<String>) -> Self {
        AoCObject::Directory {
            files: HashMap::new(),
            dirs: HashMap::new(),
            name,
            parent,
        }
    }
    fn new_cd(dest: String) -> Self {
        AoCObject::CD { next_dir: dest }
    }
    fn new_ls () -> Self {
        AoCObject::LS
    }
}
struct DaySeven {
    out: AoCObject
}
impl DaySeven {
    fn parse(filename: String) {
        let mut file = File::open(&filename).expect(&format!("Unable to open {}", filename));
        let mut f_string = String::new();
        file.read_to_string(&mut f_string)
            .expect("unable to parse the file to a string");
        let re = Regex::new(r"(\$ (?P<cmd>cd|ls) ?(?P<dest_dir>[\w\\/\.]{1,20})?)|(?P<object>((?P<folder>dir) (?P<name>\w{1,20}))|(?P<fsize>\d{1,20}) (?P<fname>[\w.]{1,20}))").unwrap();
        let mut lines: Vec<AoCObject> = Vec::new();
        for cap in re.captures_iter(&f_string) {
            match cap.name("cmd"){
                None => {},
                Some(cmd) => {
                    match cmd.as_str() {
                        "ls" => {
                            lines.push(AoCObject::new_ls());
                            continue;
                        }
                        "cd" => {
                            lines.push(AoCObject::new_cd(cap.name("dest_dir").unwrap().as_str().to_string()));
                            continue;
                        }
                        _ => {}
                    }
                }
            }
            match cap.name("folder") {
                None => {},
                Some(_) => {
                    lines.push(AoCObject::new_dir(cap.name("name").unwrap().as_str().to_string(), None));
                    continue
                }
            }
            match cap.name("fname") {
                None => {},
                Some(n) => {
                    lines.push(AoCObject::new_file(n.as_str().to_string(), cap.name("fsize").unwrap().as_str().parse::<u32>().unwrap()));
                    continue
                }
            }

        }
        for line in lines {
            println!("{}", line);
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_7() {
        let dir = AoCObject::new_dir("/".to_string(), None);
        DaySeven::parse("input/day_7_input.txt".to_string());
        assert_eq!(0, 0)
    }
}