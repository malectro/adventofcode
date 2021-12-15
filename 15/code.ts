import {decodeText, chunkLines, toArray, range} from '../iter.ts';

const directions = [
  {x: 1, y: 0},
  //{x: -1, y: 0},
  {x: 0, y: 1},
  //{x: 0, y: -1},
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

console.log('big grid', bigGrid);

const pathGrid = grid.map((row) => row.map((n) => null));
const path = [{x: 0, y: 0}];

const [bestScore, bestPath] = findBestPath(grid, gridSize, pathGrid, path);

//console.log('gridSize', gridSize, grid);
console.log('best score', bestScore);

function findBestPath(
  grid: number[][],
  gridSize: Point,
  pathGrid: Array<Array<[number, Point[]] | null>>,
  path: Point[],
): [number, Point[]] {
  const currentPoint = path[path.length - 1];

  //console.log('checking point', currentPoint);

  if (currentPoint.x === gridSize.x - 1 && currentPoint.y === gridSize.y - 1) {
    return [0, path.slice()];
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
        pathGrid[nextLocation.y][nextLocation.x] = cachedScore;
      }

      let [score, subPath] = cachedScore;
      score += grid[nextLocation.y][nextLocation.x];

      if (score < min) {
        min = score;
        bestPath = subPath;
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
  return grid.map((row) => row.map((v) => (v + inc) % 9));
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