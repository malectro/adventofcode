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

//console.log('big grid', bigGrid);

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

/*
const [bestScore, bestPath] = findBestPath(
  bigGrid,
  bigGridSize,
  scoreGrid,
  path,
);

//console.log('gridSize', gridSize, grid);
const pathGrid = bigGrid.map((row) => row.slice().fill(false));
for (const point of bestPath) {
  pathGrid[point.y][point.x] = true;
}
*/

/*
console.log(bestPath.map(
  point => `${point.x},${point.y}`
).join('-'));
*/

//console.log('best score', bestScore.toString());

printGrid(grid);
console.log(
  bigGrid
    .map((row, y) =>
      row
        .map((risk, x) => {
          //return pathGrid[y][x] ? risk : 0;
          return risk;
        })
        .join(''),
    )
    .join('\n'),
  '\n',
);

//printGrid(bigGrid);

console.log(
  scoreGrid.map(
    row => row.map((stuff) => {
      const score = stuff ? stuff : 9999;
      return padNumber(score, 5);
    }).join('|'),
  ).join('\n'),
  '\n',
);
//console.log('best score', bestScore);

/*
function findBestPath(
  grid: number[][],
  gridSize: Point,
  path: Point[],
  scoreGrid: number[][],
) {
  const currentPoint = path[path.length - 1];
  const prevPoint = path[path.length - 2];

  const score = grid[currentPoint.y][currentPoint.x];
  if (currentPoint.x === gridSize.x - 1 && currentPoint.y === gridSize.y - 1) {
    return score;
  }

  for (const direction of directions) {
    const nextPoint = addPoint(currentPoint, direction);

    if (
      isInBounds(nextLocation, gridSize) &&
      (!prevPoint ||
        (prevPoint.x !== nextPoint.x && prevPoint.y !== nextPoint.y))
    ) {
      
    }
  }
}
*/

function findBestPath(
  grid: number[][],
  gridSize: Point,
  pathGrid: Array<Array<[number, Point[]] | null>>,
  path: Point[],
): [number, Point[]] {
  const currentPoint = path[path.length - 1];

  //console.log('checking point', currentPoint);

  if (currentPoint.x === gridSize.x - 1 && currentPoint.y === gridSize.y - 1) {
    return [0, []];
  }

  let min = Infinity;
  let bestPath: Point[] = [];
  for (const direction of directions) {
    let nextLocation = addPoint(currentPoint, direction);

    if (isInBounds(nextLocation, gridSize)) {
      path.push(nextLocation);

      let cachedScore = pathGrid[nextLocation.y][nextLocation.x];

      if (cachedScore == null) {
        cachedScore = findBestPath(grid, gridSize, pathGrid, path);
        cachedScore[0] += grid[nextLocation.y][nextLocation.x];
        pathGrid[nextLocation.y][nextLocation.x] = cachedScore;
      }

      let [score, subPath] = cachedScore;

      if (score <= min) {
        min = score;
        bestPath = [nextLocation, ...subPath];
      }

      path.pop();
    }
  }

  return [min, bestPath];
}

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

/*
class Array2d<T> {
  readonly size: Point;

  constructor(size: Point, default: T) {
    this._size = size;
    this._array = Array(size.x).fill([]).map(
      _ => Array(size.y).fill(default),
    );
  }

  set(x: number, y: number, value: T) {
    this._array[x][y] = value;
  }
}
*/
