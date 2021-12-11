import {decodeText, chunkLines} from '../iter.ts';

const stdin = chunkLines(decodeText(Deno.iter(Deno.stdin)));

let grid: Array<Array<{
  level: number;
  hasFlashed: boolean;
}>> = [];
for await (const line of stdin) {
  const row = [];
  for (const char of line) {
    row.push({
      level: parseInt(char),
      hasFlashed: false,
    });
  }
  grid.push(row);
}

let count = 0;
for (let i = 0; i < 100; i++) {
  for (const row of grid) {
    for (const jelly of row) {
      jelly.level += 1;
      jelly.hasFlashed = false;
    }
  }

  for (const [y, row] of grid.entries()) {
    for (const [x, jelly] of row.entries()) {
      if (jelly.level > 9) {
        count += flash(x, y);
      }
    }
  }
}

console.log('count', count);

function flash(x: number, y: number): number {
  let count = 1;

  const jelly = grid[y][x];
  jelly.hasFlashed = true;
  jelly.level = 0;

  for (let i = Math.max(0, x - 1); i < Math.min(x + 2, grid.length); i++) {
    for (let j = Math.max(0, y - 1); j < Math.min(y + 2, grid.length); j++) {
      const jelly = grid[j][i];

      if (!jelly.hasFlashed) {
        jelly.level += 1;
        if (jelly.level > 9) {
          count += flash(i, j);
        }
      }
    }
  }

  return count;
}
