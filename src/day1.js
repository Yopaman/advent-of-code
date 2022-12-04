console.log(require('fs').readFileSync("./inputs/day1.txt", {encoding: "utf8", flag: "r"}).trim().split("\n\n").map(i => i.split("\n").reduce((acc, a) => acc + parseInt(a), 0)).sort().reverse()[1])

console.log(require('fs').readFileSync("./inputs/day1.txt", {encoding: "utf8", flag: "r"}).trim().split("\n\n").map(i => i.split("\n").reduce((acc, a) => acc + parseInt(a), 0)).sort().reverse().slice(1,4).reduce((acc, x) => acc +x, 0))
