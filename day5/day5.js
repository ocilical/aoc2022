var fs = require("fs");
function getStacks(input) {
    var stacks = [[], [], [], [], [], [], [], [], []];
    for (var i = 7; i >= 0; i--) {
        var crates = input[i].match(/^(...) (...) (...) (...) (...) (...) (...) (...) (...)/);
        if (crates) {
            for (var j = 0; j < stacks.length; j++) {
                var char = crates[j + 1][1];
                if (char != " ") {
                    stacks[j].push(char);
                }
            }
        }
    }
    return stacks;
}
function part1(input) {
    var stacks = getStacks(input);
    for (var i = 10; i < input.length; i++) {
        var instruction = input[i].match(/move (\d+) from (\d+) to (\d+)/);
        if (instruction) {
            for (var j = 0; j < parseInt(instruction[1]); j++) {
                var temp = stacks[parseInt(instruction[2]) - 1].pop();
                if (temp) {
                    stacks[parseInt(instruction[3]) - 1].push(temp);
                }
            }
        }
    }
    return stacks.map(function (a) { return a[a.length - 1]; }).join("");
}
function part2(input) {
    var stacks = getStacks(input);
    for (var i = 10; i < input.length; i++) {
        var instruction = input[i].match(/move (\d+) from (\d+) to (\d+)/);
        if (instruction) {
            var temp = [];
            for (var j = 0; j < parseInt(instruction[1]); j++) {
                var crate = stacks[parseInt(instruction[2]) - 1].pop();
                if (crate) {
                    temp.push(crate);
                }
            }
            for (var j = 0; j < parseInt(instruction[1]); j++) {
                var crate = temp.pop();
                if (crate) {
                    stacks[parseInt(instruction[3]) - 1].push(crate);
                }
            }
        }
    }
    return stacks.map(function (a) { return a[a.length - 1]; }).join("");
}
var input = fs.readFileSync("input", { encoding: 'utf8', flag: 'r' }).split("\n");
console.log("part 1: ".concat(part1(input)));
console.log("part 2: ".concat(part2(input)));
