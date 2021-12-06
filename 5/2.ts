const file = await Deno.readTextFile(new URL('input', import.meta.url));

const lines = file
  .split('\n')
  .map((text) => text.match(/(\d+),(\d+) -> (\d+),(\d+)/))
  .filter((line): line is RegExpMatchArray => line != null)
  .map(([_, x1, y1, x2, y2]) => [
    {x: parseInt(x1), y: parseInt(y1)},
    {x: parseInt(x2), y: parseInt(y2)},
  ]);

const area: Array<number[]> = [];
for (const [a, b] of lines) {
  let xRange = range(a.x, b.x);
  let yRange = range(a.y, b.y);

  if (xRange.length === 1) {
    xRange = yRange.map(v => xRange[0]);
  }

  if (yRange.length === 1) {
    yRange = xRange.map(v => yRange[0]);
  }

  for (const [x, y] of zip(xRange, yRange)) {
    if (!area[y]) {
      area[y] = [];
    }

    if (!area[y][x]) {
      area[y][x] = 0;
    }

    area[y][x]++;
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

function print(a: number[][]) {
  for (let row of a) {
    console.log(row.map((val) => (val ? val : '*')).join(' '));
  }
}

function range(from: number, to: number) {
  let result = [];

  let distance = to - from;
  let sign = Math.sign(distance);
  let size = Math.abs(distance);

  for (let i = 0; i <= size; i++) {
    result[i] = from + i * sign;
  }

  return result;
}

function zip<T>(a: T[], b: T[]) {
  let result = [];
  for (let i = 0; i < a.length; i++) {
    result.push([a[i], b[i]]);
  }
  return result;
}
