import { streamInputLines } from './util.ts';
import { range } from './iter.ts';

const schematic = [];

for await (const line of streamInputLines()) {
  schematic.push(line); 
}

let sum = 0;

for (const [y, row] of schematic.entries()) {
  for (const match of row.matchAll(/(\d+)/g)) {
    const { index } = match;
    for (const i of range(Math.max(index - 1, 0), Math.min(index + match[0].length + 1, row.length))) {
      for (const j of range(Math.max(y - 1, 0), Math.min(y + 2, schematic.length))) {
        if (schematic[j][i].match(/[^\.\d]/)) {
          sum += Number(match[0]); 
        }
      }
    }
  }
}

console.log('Part 1', sum);
