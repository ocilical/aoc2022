use std::cmp::max;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Valve {
    flow: i32,
    tunnels: Vec<String>,
}

fn parse_maze(input: &str) -> HashMap<String, Valve> {
    let re = Regex::new(r"Valve (..)[^\d]+(\d+);[ a-z]+(.*)$").unwrap();
    let mut res = HashMap::new();

    for valve in input.lines() {
        if let Some(caps) = re.captures(valve) {
            let name = caps.get(1).unwrap().as_str().to_string();
            let flow = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let tunnels = caps
                .get(3)
                .unwrap()
                .as_str()
                .split(", ")
                .map(|t| t.to_string())
                .collect::<Vec<String>>();
            res.insert(name, Valve { flow, tunnels });
        } else {
            panic!("bad format");
        }
    }

    res
}

fn part1(input: &str) -> i32 {
    let maze = parse_maze(input);
    let mut memo = HashMap::new();
    let mut visited = HashSet::new();
    explore(&mut memo, &maze, 30, &"AA".to_string(), &mut visited)
}

fn explore(
    memo: &mut HashMap<(String, i32, String), i32>,
    maze: &HashMap<String, Valve>,
    time: i32,
    curr: &String,
    open: &mut HashSet<String>,
) -> i32 {
    //println!("memo: {:?}\ntime: {}\ncurr: {}\nopen: {:?}", memo, time, curr, open);

    if time <= 0 {
        return 0;
    }

    if let Some(ans) = memo.get(&(curr.to_string(), time, open.iter().sorted().join(","))) {
        return *ans;
    }

    let curr_valve = maze.get(curr).unwrap();
    let mut res = 0;

    for valve in curr_valve.tunnels.iter() {
        res = max(res, explore(memo, maze, time - 1, valve, open))
    }
    // with turning valve
    if !open.contains(curr) && curr_valve.flow > 0 {
        open.insert(curr.to_string());
        for valve in curr_valve.tunnels.iter() {
            res = max(res, ((time - 1) * curr_valve.flow) + explore(memo, maze, time - 2, valve, open));
        }
        open.remove(curr);
    }

    memo.insert((curr.to_string(), time, open.iter().sorted().join(",")), res);
    // without turning on valve

    res
}

fn main() {
    let input = include_str!("../input");
    println!("{:?}", part1(input));
}
