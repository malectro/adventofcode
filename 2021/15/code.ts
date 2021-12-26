import {decodeText, chunkLines, toArray, range} from '../iter.ts';


class PriorityQueue<T> {
  private array: Array<T> = [];

  comparitor: (a: T) => number = Number;

  add(a: T) {
    const score = this.comparitor(a);
    const index = this.array.findIndex(
      b => score < this.comparitor(b),
    );
    if (index > -1) {
      this.array.splice(index, 0, a);
    } else {
      this.array.push(a);
    }
  }

  next(): T | void {
    return this.array.shift();
  }
}

const directions = [
  {x: 1, y: 0},
  {x: -1, y: 0},
  {x: 0, y: 1},
  {x: 0, y: -1},
];

const stdin = chunkLines(decodeText(Deno.iter(Deno.stdin)));

const grid = (await toArray(stdin)).map((line) =>
  line
    .trim()
    .split('')
    .map((n) => parseInt(n)),
);
const gridSize = {
  x: grid[0].length,
  y: grid.length,
};

const bigGrid = Array(grid.length * 5)
  .fill([])
  .map((_) => Array(gridSize.x * 5).fill(0));

for (const i of range(0, 5)) {
  for (const j of range(0, 5)) {
    projectGrid(
      bigGrid,
      {x: i * gridSize.x, y: j * gridSize.y},
      incrementGrid(grid, i + j),
    );
  }
}

const bigGridSize = {
  x: gridSize.x * 5,
  y: gridSize.y * 5,
};

const scoreGrid = bigGrid.map((row) => row.map((n) => null));
const path = [{x: 0, y: 0}];
// @ts-ignore
scoreGrid[0][0] = 0;

const queue = new PriorityQueue<{
  point: Point;
  score: number;
}>();
queue.comparitor = ({score}) => score;
queue.add({
  point: {x: 0, y: 0},
  score: 0,
});

let item;
while ((item = queue.next())) {
  const {point: current} = item;
  if (current.x === bigGridSize.x - 1 && current.y === bigGridSize.y - 1) {
    break;
  }

  for (const direction of directions) {
    const next = addPoint(current, direction);

    if (isInBounds(next, bigGridSize)) {
      const score = (scoreGrid[current.y]?.[current.x] ?? 0) + bigGrid[next.y][next.x];
      const nextScore = scoreGrid[next.y][next.x];
      if (nextScore == null || score < nextScore) {
        // @ts-ignore
        scoreGrid[next.y][next.x] = score;
        queue.add({
          point: next,
          score,
        });
      }
    }
  }
}

const finalScore = scoreGrid[bigGridSize.y - 1][bigGridSize.x - 1];
console.log('final score', finalScore);

type Point = {
  x: number;
  y: number;
};

function addPoint(point1: Point, point2: Point): Point {
  return {
    x: point1.x + point2.x,
    y: point1.y + point2.y,
  };
}

function isInBounds(point: Point, size: Point): boolean {
  return point.x >= 0 && point.y >= 0 && point.x < size.x && point.y < size.y;
}

function incrementGrid(grid: number[][], inc: number): number[][] {
  return grid.map((row) =>
    row.map((v) => v + inc).map((v) => (v > 9 ? v - 9 : v)),
  );
}

function projectGrid(
  bigGrid: number[][],
  origin: Point,
  smallGrid: number[][],
) {
  for (const y of range(0, smallGrid.length)) {
    let row = smallGrid[y];
    for (const x of range(0, row.length)) {
      bigGrid[origin.y + y][origin.x + x] = smallGrid[y][x];
    }
  }
}

function printGrid(grid: number[][]) {
  console.log(grid.map((row) => row.join('')).join('\n'), '\n');
}

function padNumber(number: number, size: number): string {
  let string = number.toString();

  for (const _ of range(string.length, size)) {
    string = '0' + string;
  }

  return string;
}
