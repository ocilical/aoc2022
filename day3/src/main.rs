use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input").unwrap();
    let priorities = input
        .lines()
        .map(|line| {
            line.bytes()
                .map(|item| {
                    if (item as char).is_uppercase() {
                        item - 38
                    } else {
                        item - 96
                    }
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    let mut sum: u32 = 0;

    for sack in &priorities {
        let mut comp1: HashSet<u8> = HashSet::new();
        let mut comp2: HashSet<u8> = HashSet::new();
        let len = sack.len();
        for (i, item) in sack.iter().enumerate() {
            if i < len / 2 {
                comp1.insert(*item);
            } else {
                comp2.insert(*item);
            }
        }
        let shared = *comp1.intersection(&comp2).next().unwrap() as u32;
        sum += shared;
    }
    print!("part 1: ");
    println!("{}", sum);

    sum = 0;
    for group in priorities.chunks(3) {
        let set1: HashSet<u8> = HashSet::from_iter(group[0].iter().cloned());
        let set2: HashSet<u8> = HashSet::from_iter(group[1].iter().cloned());
        let set3: HashSet<u8> = HashSet::from_iter(group[2].iter().cloned());
        let inter1: HashSet<u8> = HashSet::from_iter(set1.intersection(&set2).cloned());
        let mut inter2 = inter1.intersection(&set3).cloned();
        let shared = inter2.next().unwrap() as u32;
        sum += shared;
    }
    print!("part 2: ");
    println!("{}", sum);
}
