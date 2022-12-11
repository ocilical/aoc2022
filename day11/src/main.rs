use lazy_static::lazy_static;
use num::integer;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Op {
    Add(Rh),
    Mul(Rh),
}

#[derive(Debug, Clone)]
enum Rh {
    Num(i64),
    Old,
}

#[derive(Debug, Clone)]
struct Monkey {
    inspects: i64,
    items: Vec<i64>,
    operation: Op,
    test: i64,
    ttar: usize,
    ftar: usize,
}

impl FromStr for Monkey {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"[^\n]+\n[^\n\d]+((?:\d|,| )+)\n[^\+\*]+(\+|\*) ([^\n]+)\n[^\d]+(\d+)\n[^\d]+(\d+)\n[^\d]+(\d+)")
                .unwrap();
        }
        if let Some(caps) = RE.captures(s) {
            Ok(Monkey {
                inspects: 0,
                items: caps
                    .get(1)
                    .unwrap()
                    .as_str()
                    .split(", ")
                    .map(|x| x.parse().unwrap())
                    .collect(),
                operation: if caps.get(2).unwrap().as_str() == "*" {
                    Op::Mul(if caps.get(3).unwrap().as_str() == "old" {
                        Rh::Old
                    } else {
                        Rh::Num(caps.get(3).unwrap().as_str().parse().unwrap())
                    })
                } else {
                    Op::Add(if caps.get(3).unwrap().as_str() == "old" {
                        Rh::Old
                    } else {
                        Rh::Num(caps.get(3).unwrap().as_str().parse().unwrap())
                    })
                },
                test: caps.get(4).unwrap().as_str().parse().unwrap(),
                ttar: caps.get(5).unwrap().as_str().parse().unwrap(),
                ftar: caps.get(6).unwrap().as_str().parse().unwrap(),
            })
        } else {
            Err("bad monkey format".to_string())
        }
    }
}

fn simulate(input: &str, rounds: i32, part2: bool) -> i64 {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|s| s.parse::<Monkey>().unwrap())
        .collect();
    let lcm = monkeys
        .iter()
        .map(|m| m.test)
        .reduce(|a, b| integer::lcm(a, b))
        .unwrap();
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let mut items = monkeys[i].items.to_owned();
            monkeys[i].items = vec![];
            for item in items.iter_mut() {
                monkeys[i].inspects += 1;
                match monkeys[i].operation {
                    Op::Add(Rh::Num(x)) => *item += x,
                    Op::Add(Rh::Old) => *item += *item,
                    Op::Mul(Rh::Num(x)) => *item *= x,
                    Op::Mul(Rh::Old) => *item *= *item,
                }
                if !part2 {
                    *item /= 3
                } else {
                    *item %= lcm
                };
                let target = if *item % monkeys[i].test == 0 {
                    monkeys[i].ttar
                } else {
                    monkeys[i].ftar
                };
                monkeys[target].items.push(*item);
            }
        }
    }
    let mut monkeys_sorted = monkeys.to_vec();
    monkeys_sorted.sort_by(|a, b| b.inspects.cmp(&a.inspects));
    monkeys_sorted[0].inspects * monkeys_sorted[1].inspects
}

fn main() {
    let input = include_str!("../input");
    println!("part 1: {}", simulate(input, 20, false));
    println!("part 2: {}", simulate(input, 10000, true))
}
