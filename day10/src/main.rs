use image::{imageops, ImageBuffer, Rgb, RgbImage};
use std::fs;

fn part1(input: String) -> Vec<i32> {
    let instr = input.lines().collect::<Vec<&str>>();
    let mut ip = 0;
    let mut reg = 1;
    let mut cycles = 1;
    let mut working = false;
    let mut signals: Vec<i32> = Vec::new();

    while ip < instr.len() {
        if instr[ip] == "noop" {
            ip += 1;
        } else if !working {
            working = true;
        } else {
            let imm = instr[ip].split(" ").collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();
            reg += imm;
            ip += 1;
            working = false;
        }
        cycles += 1;

        if (cycles - 20) % 40 == 0 {
            signals.push(cycles * reg);
        }
    }
    signals
}

fn part2(input: String) -> RgbImage {
    let instr = input.lines().collect::<Vec<&str>>();
    let mut ip = 0;
    let mut reg: i32 = 1;
    let mut cycles = 0;
    let mut working = false;
    let mut res: RgbImage = ImageBuffer::new(40, 6);
    let mut vpos = 0;

    while ip < instr.len() {
        // draw pixels at start of cycle
        let hpos = cycles % 40;
        if hpos == 0 && cycles != 0 {
            vpos += 1;
        }
        if reg - 1 == hpos || reg == hpos || reg + 1 == hpos {
            res.put_pixel(hpos as u32, vpos, Rgb([255, 255, 255]));
        } else {
            res.put_pixel(hpos as u32, vpos, Rgb([0, 0, 0]));
        }

        if instr[ip] == "noop" {
            ip += 1;
        } else if !working {
            working = true;
        } else {
            let imm = instr[ip].split(" ").collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();
            reg += imm;
            ip += 1;
            working = false;
        }

        cycles += 1;
    }
    res
}

fn main() {
    let signals = part1(fs::read_to_string("input").unwrap());
    let total: i32 = signals.iter().sum();
    println!("part 1: {}", total);

    let mut im = part2(fs::read_to_string("input").unwrap());
    im = imageops::resize(&im, 40 * 4, 6 * 4, imageops::FilterType::Nearest);
    let _ = im.save("result.png");
}
