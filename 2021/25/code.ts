import {
  decodeText,
  chunkLines,
  map,
  range,
  clamp,
  toArray,
  takeWhile,
} from '../iter.ts';

const lines = await toArray(chunkLines(decodeText(Deno.iter(Deno.stdin))));

let grid = lines.map(
  line => Array.from(line),
);

let steps = 0;
while (true) {
  steps++;

  let hasChanged = false;
  for (const {type, direction} of [
    {type: '>', direction: {x: 1, y: 0}},
    {type: 'v', direction: {x: 0, y: 1}},
  ]) {
    const newGrid = structuredClone(grid); 
    for (const [y, row] of grid.entries()) {
      for (const [x, value] of row.entries()) {
        if (value === type) {
          const pos = {
            x: (x + direction.x) % row.length,
            y: (y + direction.y) % grid.length,
          };
          if (grid[pos.y][pos.x] === '.') {
            hasChanged = true;
            [newGrid[y][x], newGrid[pos.y][pos.x]] = [newGrid[pos.y][pos.x], newGrid[y][x]];
          }
        }
      }
    }
    grid = newGrid;
  }

  if (!hasChanged) {
    break;
  }
}

console.log('number of steps', steps);

function serialize(grid: string[][]): string {
  return grid.map(row => row.join('')).join('\n');
}
