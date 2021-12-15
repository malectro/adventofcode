import {decodeText, chunkLines, toArray} from '../iter.ts';

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

const pathGrid = grid.map((row) => row.map((n) => false));
const path = [{x: 0, y: 0}];
pathGrid[0][0] = true;

const [bestScore, bestPath] = findBestPath(grid, gridSize, pathGrid, path);

//console.log('gridSize', gridSize, grid);
console.log('best score', bestScore, bestPath);

function findBestPath(
  grid: number[][],
  gridSize: Point,
  pathGrid: boolean[][],
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

    if (
      isInBounds(nextLocation, gridSize) &&
      !pathGrid[nextLocation.y][nextLocation.x]
    ) {
      pathGrid[nextLocation.y][nextLocation.x] = true;
      path.push(nextLocation);
      let [score, subPath] = findBestPath(grid, gridSize, pathGrid, path);
      score += grid[nextLocation.y][nextLocation.x];
      if (score < min) {
        min = score;
        bestPath = subPath;
      }
      path.pop();
      pathGrid[nextLocation.y][nextLocation.x] = false;
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
