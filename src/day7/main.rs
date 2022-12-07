use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::str::FromStr;
use std::cmp::min;
use crate::ParsedLine::{Dir, File};

const INPUT: &str = include_str!("input.txt");

const MAX_SIZE_DIR: usize = 100000;

const TOTAL_DISK: usize = 70000000;
const REQUIRED: usize = 30000000;

enum Command {
    CdPrev,
    Cd(String),
    Ls,
}

impl FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(" ").collect();
        match split[1] {
            "cd" => {
                match split[2] {
                    ".." => Ok(Command::CdPrev),
                    &_ => Ok(Command::Cd(split[2].into()))
                }
            }
            "ls" => Ok(Command::Ls),
            _ => Err(())
        }
    }
}

#[derive(Debug)]
enum ParsedLine {
    Dir(String),
    File(String, usize),
}

#[derive(Default, Debug)]
struct Directory {
    subdirs: HashMap<String, Directory>,
    files: Vec<(String, usize)>,
    size: usize,
}

impl Directory {
    fn create_file(&mut self, name: String, size: usize, path: &Vec<String>) {
        let mut current = self;
        for p in 0..path.len() {
            current.size += size;
            current = current.subdirs.entry(path[p].to_owned()).or_default()
        }
        current.size += size;
        current.files.push((name, size))
    }

    fn mkdir(&mut self, name: String, path: &Vec<String>) {
        let mut current = self;
        for p in 0..path.len() {
            current = current.subdirs.entry(path[p].to_owned()).or_default()
        }
        current.subdirs.entry(name).or_default();
    }

    fn iter_with_operation<F: FnMut(&mut Directory)>(&mut self, mut f: F) {
        let mut queue = VecDeque::new();
        queue.push_back(self);

        while let Some(top) = queue.pop_back() {
            f(top);
            for c in top.subdirs.iter_mut() { queue.push_back(c.1); }
        }
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut fs = Directory::default();

    let mut path: Vec<String> = Vec::new();

    for i in INPUT.lines() {
        if i.starts_with("$") {
            match Command::from_str(i).unwrap() {
                Command::CdPrev => { path.pop(); }
                Command::Cd(p) => path.push(p),
                _ => {}
            }
            continue;
        }
        match parse_line(i)? {
            Dir(d) => fs.mkdir(d, &path),
            File(n, s) => fs.create_file(n, s, &path)
        }
    }

    let mut result = 0;
    fs.iter_with_operation(|top| { if top.size < MAX_SIZE_DIR { result += top.size } });
    println!("part1 = {:?}", result);

    let remaining = TOTAL_DISK - fs.size;
    result = TOTAL_DISK;
    fs.iter_with_operation(|top| { if top.size + remaining >= REQUIRED { result = min(result, top.size) } });

    println!("part2 = {:?}", result);

    Ok(())
}

fn parse_line(line: &str) -> Result<ParsedLine, Box<dyn Error>> {
    let line_content: Vec<&str> = line.trim().split(" ").collect();

    Ok(match line_content[0] {
        "dir" => Dir(line_content[1].into()),
        _ => File(line_content[1].into(), line_content[0].parse::<usize>()?)
    })
}