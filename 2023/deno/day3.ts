import { streamInputLines } from "./util.ts";
import { range } from "./iter.ts";

const schematic = [];
const gears = new Map();

for await (const line of streamInputLines()) {
  schematic.push(line);
}

let sum = 0;

for (const [y, row] of schematic.entries()) {
  for (const match of row.matchAll(/(\d+)/g)) {
    const { index } = match;
    for (
      const i of range(
        Math.max(index - 1, 0),
        Math.min(index + match[0].length + 1, row.length),
      )
    ) {
      for (
        const j of range(Math.max(y - 1, 0), Math.min(y + 2, schematic.length))
      ) {
        if (schematic[j][i].match(/[^\.\d]/)) {
          const number= Number(match[0]);
          sum += number;

          if (schematic[j][i] === '*') {
            const gearKey = i + j * row.length;
            let group = gears.get(gearKey);
            if (!group) {
              group = [];
              gears.set(gearKey, group);
            }
            group.push(number);
          }
        }
      }
    }
  }
}

console.log("Part 1", sum);

const part2 = [...gears.values()].filter(
  (group) => group.length > 1,
).map((group) => group.reduce((acc, val) => acc * val, 1)).reduce(
  (acc, val) => acc + val,
  0,
);

console.log("Part 2", part2);
