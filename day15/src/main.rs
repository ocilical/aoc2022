use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashSet;

#[derive(Debug)]
struct Sensor {
    pos: (i64, i64),
    radius: i64,
}

#[inline(always)]
fn manhatten_dist(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn parse_sensors(input: &str) -> (Vec<Sensor>, HashSet<(i64, i64)>) {
    let re =
        Regex::new(r"[^\d-]+((?:\d|-)+)[^\d-]+((?:\d|-)+)[^\d-]+((?:\d|-)+)[^\d-]+((?:\d|-)+)")
            .unwrap();
    let mut sensors = Vec::new();
    let mut beacons = HashSet::new();

    for sensor in input.lines() {
        if let Some(caps) = re.captures(sensor) {
            let s_pos: (i64, i64) = (
                caps.get(1).unwrap().as_str().parse().unwrap(),
                caps.get(2).unwrap().as_str().parse().unwrap(),
            );
            let b_pos: (i64, i64) = (
                caps.get(3).unwrap().as_str().parse().unwrap(),
                caps.get(4).unwrap().as_str().parse().unwrap(),
            );
            beacons.insert(b_pos);
            sensors.push(Sensor {
                pos: s_pos,
                radius: manhatten_dist(s_pos, b_pos),
            })
        }
    }

    (sensors, beacons)
}

fn part1(input: &str, y: i64) -> i64 {
    let mut res = 0;
    let (sensors, beacons) = parse_sensors(input);
    let min = sensors.iter().map(|s| s.pos.0 - s.radius).min().unwrap();
    let max = sensors.iter().map(|s| s.pos.0 + s.radius).max().unwrap();

    for x in min..=max {
        if sensors
            .iter()
            .any(|s| manhatten_dist((x, y), s.pos) <= s.radius && !beacons.contains(&(x, y)))
        {
            res += 1;
        }
    }

    res
}

fn part2(input: &str, max_y: i64, max_x: i64) -> i64 {
    let (sensors, _) = parse_sensors(input);

    for y in 0..max_y {
        // get range of each sensor
        let mut ranges = Vec::new();
        for sensor in sensors.iter() {
            let reach = sensor.radius - (y - sensor.pos.1).abs();
            if reach < 0 || sensor.pos.0 - reach < 0 && sensor.pos.0 + reach < 0 {
                continue;
            }
            let r0 = sensor.pos.0 - reach;
            let r1 = sensor.pos.0 + reach;
            ranges.push((min(r0, r1), max(r0, r1)));
        }

        ranges.sort();

        // find if there are any holes in the range
        let mut front = 0;
        for range in ranges {
            if front > max_x {
                break;
            }
            if range.0 <= front {
                front = max(front, range.1);
            } else {
                println!("found it! {:?}", (front + 1, y));
                return ((front + 1) * 4000000) + y;
            }
        }
    }

    unreachable!();
}

fn main() {
    let input = include_str!("../input");
    println!("part 1: {}", part1(input, 2000000));
    println!("part 2: {}", part2(input, 4000000, 4000000));
}
