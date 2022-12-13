use std::cmp::Ordering;

use serde_json::Value;

#[derive(Debug)]
struct Trans {
    val: Value,
}

impl Trans {
    fn new(val: Value) -> Trans {
        Trans { val: val }
    }
}

impl Ord for Trans {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        compare_val(&self.val, &other.val)
    }
}

impl PartialOrd for Trans {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Trans {
    fn eq(&self, other: &Self) -> bool {
        if let Ordering::Equal = compare_val(&self.val, &other.val) {
            true
        } else {
            false
        }
    }
}
impl Eq for Trans {}

fn to_trans(input: &str) -> Vec<Trans> {
    input
        .lines()
        .filter(|p| *p != "")
        .map(|p| Trans::new(serde_json::from_str::<Value>(p).unwrap()))
        .collect()
}

fn get_packets(input: &str) -> Vec<(Trans, Trans)> {
    input
        .split("\n\n")
        .map(|p| {
            let mut pair = p.lines().map(|s| serde_json::from_str::<Value>(s).unwrap());
            (
                Trans::new(pair.next().unwrap()),
                Trans::new(pair.next().unwrap()),
            )
        })
        .collect::<Vec<(Trans, Trans)>>()
}

fn is_sorted(packet: (Trans, Trans)) -> bool {
    packet.0 <= packet.1
}

fn compare_val(left: &Value, right: &Value) -> Ordering {
    let left = left.clone();
    let right = right.clone();
    // if both numbers
    if let Value::Number(ref l) = left {
        if let Value::Number(ref r) = right {
            if r.as_u64().unwrap() < l.as_u64().unwrap() {
                return Ordering::Greater;
            } else if r.as_u64().unwrap() == l.as_u64().unwrap() {
                return Ordering::Equal;
            } else {
                return Ordering::Less;
            }
        }
    }

    let l = match left {
        Value::Number(_) => vec![left],
        Value::Array(_) => left.as_array().unwrap().to_vec(),
        _ => unimplemented!(),
    };

    let r = match right {
        Value::Number(_) => vec![right],
        Value::Array(_) => right.as_array().unwrap().to_vec(),
        _ => unimplemented!(),
    };

    for i in 0..l.len() {
        if i >= r.len() {
            return Ordering::Greater;
        }
        let res = compare_val(&l[i], &r[i]);
        if let Ordering::Equal = res {
            continue;
        }
        return res;
    }

    if r.len() > l.len() {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

fn part1(input: &str) -> usize {
    let packets = get_packets(input);
    packets
        .into_iter()
        .map(is_sorted)
        .enumerate()
        .map(|p| if p.1 { p.0 + 1 } else { 0 })
        .sum()
}

fn part2(input: &str) -> usize {
    let mut trans = to_trans(input);
    trans.push(Trans::new(serde_json::from_str::<Value>("[[2]]").unwrap()));
    trans.push(Trans::new(serde_json::from_str::<Value>("[[6]]").unwrap()));
    trans.sort();
    let pos1 = trans.iter().position(|x| serde_json::to_string(&x.val).unwrap() == "[[2]]".to_string());
    let pos2 = trans.iter().position(|x| serde_json::to_string(&x.val).unwrap() == "[[6]]".to_string());
    (pos1.unwrap() + 1) * (pos2.unwrap() + 1)
}

fn main() {
    let input = include_str!("../input");

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}
