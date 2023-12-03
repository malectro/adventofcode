import { enumerate } from "./iter.ts";
import { streamInputLines } from "./util.ts";

const totals = {
  red: 12,
  green: 13,
  blue: 14,
};

const maxes = { ...totals };

let part1 = 0;
let part2 = 0;
for await (const [i, line] of enumerate(streamInputLines())) {
  for (const key of Object.keys(maxes)) {
    maxes[key] = 0;
  }

  for (const match of line.matchAll(/(\d+) (\w+)/g)) {
    maxes[match[2]] = Math.max(maxes[match[2]], Number(match[1]));
  }

  if (Object.keys(maxes).every((key) => maxes[key] <= totals[key])) {
    part1 += i + 1;
  }

  part2 += maxes.red * maxes.green * maxes.blue;
}

console.log("part 1", part1);
console.log("part 2", part2);
