use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use regex::Regex;

struct AoCFile {
    name: String,
    size: u32,
}
struct Directory {
    files: HashMap::<String, AoCFile>,
    dirs: HashMap::<String, Directory>,
    name: String,
    parent: Option<String>,
}
struct ChangeDir{
    next_dir: String,
}
struct DaySeven {
    out: Directory
}
impl DaySeven {
    fn parse(filename: String) {
        let mut file = File::open(&filename).expect(&format!("Unable to open {}", filename));
        let mut f_string = String::new();
        file.read_to_string(&mut f_string)
            .expect("unable to parse the file to a string");
        let re = Regex::new(r"(?P<cd>\$ [a-z]{1,10}\s[\w\/.]{1,10}\n)|(?P<ls>\$ ls\n)|(?P<dir>dir (?P<dir_name>[a-z]{1,10}))|(?P<file>(?P<size>\d{1,10}) [\w.]{1,20})").unwrap();
        let _ = re.captures_iter(&f_string).map(|x| {

        }).collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_7() {
        let dir = Directory{
            files: HashMap::new(),
            dirs: HashMap::new(),
            name: "/".to_string(),
            parent: None,
        };
        assert_eq!(0, 0)
    }
}