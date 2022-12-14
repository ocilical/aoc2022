use std::cmp::{max, min};
use std::collections::HashMap;

type Cave = HashMap<(i32, i32), Fill>;

enum Fill {
    Rock,
    Sand,
}

fn parse_cave(input: &str) -> Cave {
    let mut res: Cave = HashMap::new();

    for l in input.lines() {
        let coords = l
            .split(" -> ")
            .map(|x| {
                let c = x.split(',').collect::<Vec<&str>>();
                (c[0].parse().unwrap(), c[1].parse().unwrap())
            })
            .collect::<Vec<(i32, i32)>>();

        for i in 1..coords.len() {
            if coords[i - 1].0 == coords[i].0 {
                for y in min(coords[i - 1].1, coords[i].1)..=max(coords[i - 1].1, coords[i].1) {
                    res.insert((coords[i].0, y), Fill::Rock);
                }
            } else {
                for x in min(coords[i - 1].0, coords[i].0)..=max(coords[i - 1].0, coords[i].0) {
                    res.insert((x, coords[i].1), Fill::Rock);
                }
            }
        }
    }

    return res;
}

#[allow(dead_code)]
fn print_cave(cave: &Cave) {
    let min_x = cave.keys().min_by_key(|x| x.0).unwrap().0;
    let max_x = cave.keys().max_by_key(|x| x.0).unwrap().0;
    let min_y = 0; // just to make it look nicer
    let max_y = cave.keys().max_by_key(|x| x.1).unwrap().1;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if x == 500 && y == 0 {
                print!("+");
                continue;
            }
            let val = cave.get(&(x, y));
            match val {
                None => print!("."),
                Some(Fill::Rock) => print!("#"),
                Some(Fill::Sand) => print!("o"),
            }
        }
        println!();
    }
}

fn part1(mut cave: Cave) -> i32 {
    let mut res = 0;
    let limit = cave.keys().max_by_key(|x| x.1).unwrap().1;

    'outer: loop {
        let mut pos = (500, 0);
        loop {
            if pos.1 > limit {
                break 'outer;
            }
            if cave.get(&(pos.0, pos.1 + 1)).is_none() {
                pos = (pos.0, pos.1 + 1);
            } else if cave.get(&(pos.0 - 1, pos.1 + 1)).is_none() {
                pos = (pos.0 - 1, pos.1 + 1);
            } else if cave.get(&(pos.0 + 1, pos.1 + 1)).is_none() {
                pos = (pos.0 + 1, pos.1 + 1);
            } else {
                cave.insert(pos, Fill::Sand);
                res += 1;
                break;
            }
        }
    }

    res
}

fn part2(mut cave: Cave) -> i32 {
    let mut res = 0;
    let limit = cave.keys().max_by_key(|x| x.1).unwrap().1 + 2;

    'outer: loop {
        let mut pos = (500, 0);
        loop {
            if cave.get(&(pos.0, pos.1 + 1)).is_none() && pos.1 + 1 < limit {
                pos = (pos.0, pos.1 + 1);
            } else if cave.get(&(pos.0 - 1, pos.1 + 1)).is_none() && pos.1 + 1 < limit {
                pos = (pos.0 - 1, pos.1 + 1);
            } else if cave.get(&(pos.0 + 1, pos.1 + 1)).is_none() && pos.1 + 1 < limit {
                pos = (pos.0 + 1, pos.1 + 1);
            } else {
                cave.insert(pos, Fill::Sand);
                res += 1;
                if pos == (500, 0) {
                    break 'outer;
                } else {
                    break;
                }
            }
        }
    }
    res
}

fn main() {
    let input = include_str!("../input");
    println!("part 1: {}", part1(parse_cave(input)));
    println!("part 2: {}", part2(parse_cave(input)));
}
