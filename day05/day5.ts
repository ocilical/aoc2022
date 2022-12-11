const fs = require("fs");

function getStacks(input: string[]): string[][] {
    let stacks: string[][] = [[], [], [], [], [], [], [], [], []];
    for (let i = 7; i >= 0; i--) {
        const crates = input[i].match(/^(...) (...) (...) (...) (...) (...) (...) (...) (...)/);
        if (crates) {
            for (let j = 0; j < stacks.length; j++) {
                const char = crates[j + 1][1];
                if (char != " ") {
                    stacks[j].push(char);
                }
            }
        }
    }
    return stacks;
}

function part1(input: string[]): string {
    let stacks = getStacks(input);
    for (let i = 10; i < input.length; i++) {
        let instruction = input[i].match(/move (\d+) from (\d+) to (\d+)/);
        if (instruction) {
            for (let j = 0; j < parseInt(instruction[1]); j++) {
                let temp = stacks[parseInt(instruction[2]) - 1].pop();
                if (temp) {
                    stacks[parseInt(instruction[3]) - 1].push(temp);
                }
            }
        }
    }

    return stacks.map(a => a[a.length - 1]).join("");
}

function part2(input: string[]): string {
    let stacks = getStacks(input);
    for (let i = 10; i < input.length; i++) {
        let instruction = input[i].match(/move (\d+) from (\d+) to (\d+)/);
        if (instruction) {
            let temp: string[] = [];
            for (let j = 0; j < parseInt(instruction[1]); j++) {
                let crate = stacks[parseInt(instruction[2]) - 1].pop();
                if (crate) {
                    temp.push(crate);
                }
            }
            for (let j = 0; j < parseInt(instruction[1]); j++) {
                let crate = temp.pop();
                if (crate) {
                    stacks[parseInt(instruction[3]) - 1].push(crate);
                }
            }
        }
    }

    return stacks.map(a => a[a.length - 1]).join("");
}

const input = fs.readFileSync("input", { encoding: 'utf8', flag: 'r' }).split("\n");
console.log(`part 1: ${part1(input)}`);
console.log(`part 2: ${part2(input)}`);
