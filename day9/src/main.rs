use std::collections::HashSet;
use std::collections::HashMap;
use std::fs;

// fn simulate(input: String, length: i32) -> usize {
//     let mut head: (i32, i32) = (0, 0);
//     let mut tail: (i32, i32) = (0, 0);
//     let mut visited = HashSet::new();
//     visited.insert(tail);

//     for inp in input.lines() {
//         let mut instr = inp.split(" ");
//         let dir = instr.next().unwrap();
//         let amt = instr.next().unwrap().parse::<i32>().unwrap();

//         for _ in 0..amt {
//             match dir {
//                 "U" => head.1 += 1,
//                 "D" => head.1 -= 1,
//                 "L" => head.0 -= 1,
//                 "R" => head.0 += 1,
//                 _ => panic!("bad format"),
//             }

//             if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
//                 // diagonal case
//                 if head.0 != tail.0 && head.1 != tail.1 {
//                     tail.0 += (head.0 - tail.0).signum();
//                     tail.1 += (head.1 - tail.1).signum();
//                 } else {
//                     if dir == "L" || dir == "R" {
//                         tail.0 += (head.0 - tail.0).signum();
//                     } else {
//                         tail.1 += (head.1 - tail.1).signum();
//                     }
//                 }
//             }

//             visited.insert(tail);
//         }
//     }
//     visited.len()
// }

fn simulate(input: String, length: usize) -> usize {
    let mut rope = vec![(0, 0); length];
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(rope[rope.len() - 1]);

    print_rope(&rope);
    println!();

    for inp in input.lines() {
        println!("== {} ==\n", inp);

        let mut instr = inp.split(" ");
        let dir = instr.next().unwrap();
        let amt = instr.next().unwrap().parse::<i32>().unwrap();

        for _ in 0..amt {
            match dir {
                "U" => rope[0].1 += 1,
                "D" => rope[0].1 -= 1,
                "L" => rope[0].0 -= 1,
                "R" => rope[0].0 += 1,
                _ => panic!("bad format"),
            }

            for i in 1..rope.len() {
                if (rope[i - 1].0 - rope[i].0).abs() > 1
                    || (rope[i - 1].1 - rope[i].1).abs() > 1
                {
                    // diagonal case
                    if rope[i - 1].0 != rope[i].0 && rope[i - 1].1 != rope[i].1 {
                        rope[i].0 += (rope[i - 1].0 - rope[i].0).signum();
                        rope[i].1 += (rope[i - 1].1 - rope[i].1).signum();
                    } else {
                        if dir == "L" || dir == "R" {
                            rope[i].0 += (rope[i - 1].0 - rope[i].0).signum();
                        } else {
                            rope[i].1 += (rope[i - 1].1 - rope[i].1).signum();
                        }
                    }
                }
            }
            print_rope(&rope);
            println!();
            visited.insert(rope[rope.len() - 1]);
        }
    }
    visited.len()
}

fn print_rope(rope: &Vec<(i32, i32)>) {
    let width = 26;
    let height = 21;
    let start_y = -5;
    let start_x = -11;
    let mut map = HashMap::new();
    for (i, pos) in rope.iter().enumerate() {
        map.entry(pos).or_insert(i.to_string());
    }

    for y in (start_y..=(start_y + height - 1)).rev() {
        for x in start_x..=(start_x + width - 1) {
            print!("{}", map.get(&(x, y)).unwrap_or(&".".to_string()))
        }
        println!();
    }
}

fn main() {
    //println!("part 1: {}", simulate(fs::read_to_string("input3").unwrap(), 2));
    println!("part 1: {}", simulate(fs::read_to_string("input3").unwrap(), 10));
}
