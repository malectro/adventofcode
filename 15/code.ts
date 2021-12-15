import {decodeText, chunkLines, toArray} from '../iter.ts';

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

const pathGrid = grid.map((row) => row.map((n) => false));
const path = [{x: 0, y: 0}];

const bestScore = findBestPath(grid, gridSize, pathGrid, path);

console.log('best score', bestScore);

function findBestPath(
  grid: number[][],
  gridSize: Point,
  pathGrid: boolean[][],
  path: Point[],
): number {
  const currentPoint = path[path.length - 1];

  //console.log('checking point', currentPoint);

  if (currentPoint.x === gridSize.x - 1 && currentPoint.y === gridSize.y - 1) {
    return 0;
  }

  let min = Infinity;
  for (const direction of directions) {
    let nextLocation = addPoint(currentPoint, direction);

    if (
      isInBounds(nextLocation, gridSize) &&
      !pathGrid[nextLocation.y][nextLocation.x]
    ) {
      pathGrid[nextLocation.y][nextLocation.x] = true;
      path.push(nextLocation);
      min = Math.min(
        grid[nextLocation.y][nextLocation.x] +
          findBestPath(grid, gridSize, pathGrid, path),
        min,
      );
      path.pop();
      pathGrid[nextLocation.y][nextLocation.x] = false;
    }
  }

  return min;
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
