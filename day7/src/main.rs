use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn calc_sizes(input: String) -> HashMap<String, i64> {
    let cd = Regex::new(r"^\$ cd (.*)$").unwrap();
    let filesize = Regex::new(r"(\d+) .*$").unwrap();
    let mut path = vec!["/"];
    let mut sizes = HashMap::new();

    for line in input.lines() {
        if let Some(caps) = cd.captures(line) {
            let dir = caps.get(1).unwrap().as_str();
            if dir == ".." {
                path.pop();
            } else if dir == "/" {
                path.clear();
                path.push("/");
            } else {
                path.push(dir);
            }
        } else if let Some(caps) = filesize.captures(line) {
            let size = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
            for i in 0..path.len() {
                let s = sizes.entry(path[0..=i].join(",")).or_insert(0);
                *s += size;
            }
        }
    }
    sizes
}

fn part1(input: String) -> i64 {
    calc_sizes(input).values().filter(|size| **size <= 100000).sum()
}

fn part2(input: String) -> i64 {
    let sizes = calc_sizes(input);
    let needed = 30000000 - (70000000 - sizes.get("/").unwrap());
    *sizes.values().filter(|size| **size >= needed).min().unwrap()
}

fn main() {
    println!("part 1: {}", part1(fs::read_to_string("input").unwrap()));
    println!("part 2: {}", part2(fs::read_to_string("input").unwrap()));
}
