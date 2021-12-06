const file = await Deno.readTextFile(new URL('input', import.meta.url));

const lines = file
  .split('\n')
  .map((text) => text.match(/(\d+),(\d+) -> (\d+),(\d+)/))
  .filter((line): line is RegExpMatchArray => line != null)
  .map(([_, x1, y1, x2, y2]) => [
    {x: parseInt(x1), y: parseInt(y1)},
    {x: parseInt(x2), y: parseInt(y2)},
  ])
  .filter(([a, b]) => a.x === b.x || a.y === b.y);

const area: Array<number[]> = [];
for (const [a, b] of lines) {
  let counter = {x: 0, y: 0};
  let origin, end;

  if (a.x < b.x || a.y < b.y) {
    origin = a;
    end = b;
  } else {
    origin = b;
    end = a;
  }

  for (let i = origin.x; i < end.x + 1; i++) {
    for (let j = origin.y; j < end.y + 1; j++) {
      if (!area[i]) {
        area[i] = [];
      }

      if (!area[i][j]) {
        area[i][j] = 0;
      }

      area[i][j]++;
    }
  }
}

let total = 0;
for (const row of area) {
  if (row) {
    for (const value of row) {
      if (value && value > 1) {
        total++;
      }
    }
  }
}

console.log('total', total);
