import {
  decodeText,
  chunkLines,
  filter,
  takeWhile,
  map,
  mapNonNullable,
  toArray,
} from '../iter.ts';

const stdin = chunkLines(decodeText(Deno.iter(Deno.stdin)));

const pointData = await toArray(
  map(
    takeWhile(stdin, (line) => line),
    (line) =>
      line
        .trim()
        .split(',')
        .map((v) => parseInt(v)),
  ),
);

const maxes = pointData
  .reduce((max, [x, y]) => [Math.max(x, max[0]), Math.max(y, max[1])], [0, 0])
  .map((v) => v + 1);
const size = {x: maxes[0], y: maxes[1]};

let grid = Array(size.x * size.y).fill(false);

for (const [x, y] of pointData) {
  grid[x + y * size.x] = true;
}

const folds = await toArray(
  map(
    mapNonNullable(stdin, (line) => line.match(/([xy])=(\d+)$/)),
    (fold) => ({
      axis: (fold[1] === 'x' ? 'x' : 'y') as 'x' | 'y',
      value: parseInt(fold[2]),
    }),
  ),
);

const {grid: finalGrid, size: finalSize} = folds.reduce(
  ({grid, size}, fold) => foldGrid(grid, size, fold),
  {
    grid,
    size,
  },
);

console.log('finalGrid', serializeGrid(finalGrid, finalSize));

function serializeGrid(grid: boolean[], size: Point): string {
  let result = '';

  for (let i = 0; i < grid.length; i += size.x) {
    result += '\n';
    for (let j = 0; j < size.x; j++) {
      result += grid[j + i] ? '#' : '.';
    }
  }

  return result;
}

type Point = {x: number; y: number};

function foldGrid(
  grid: boolean[],
  size: Point,
  fold: {axis: 'x' | 'y'; value: number},
): {grid: boolean[]; size: Point} {
  const foldPoint1 =
    fold.axis === 'x'
      ? {
          x: fold.value,
          y: size.y,
        }
      : {
          x: size.x,
          y: fold.value,
        };
  const foldPoint2 =
    fold.axis === 'x'
      ? {
          x: fold.value + 1,
          y: 0,
        }
      : {
          x: 0,
          y: fold.value + 1,
        };

  const [newGrid, newGridSize] = sliceGrid(
    grid,
    size,
    {x: 0, y: 0},
    foldPoint1,
  );
  const [extraGrid, foldedGridSize] = sliceGrid(grid, size, foldPoint2, size);
  const foldedGrid = mirror(extraGrid, foldedGridSize, fold.axis);

  return merge(newGrid, foldedGrid, newGridSize, foldedGridSize);
}

function sliceGrid(
  grid: boolean[],
  size: Point,
  from: Point,
  to: Point,
): [boolean[], Point] {
  const newSize = {
    x: to.x - from.x,
    y: to.y - from.y,
  };

  const newGrid = Array(newSize.x * newSize.y).fill(false);

  const fromY = from.y * size.x;
  const toY = to.y * size.x;

  for (let y = fromY, i = 0; y < toY; y += size.x, i += newSize.x) {
    for (let j = 0; j < newSize.x; j++) {
      newGrid[i + j] = grid[y + j + from.x];
    }
  }

  return [newGrid, newSize];
}

function mirror(grid: boolean[], size: Point, axis: 'x' | 'y'): boolean[] {
  const newGrid = grid.slice();

  for (let i = 0; i < grid.length; i += size.x) {
    for (let j = 0; j < size.x; j++) {
      if (axis === 'y') {
        newGrid[grid.length - size.x - i + j] = grid[i + j];
      } else {
        newGrid[i + size.x - 1 - j] = grid[i + j];
      }
    }
  }

  return newGrid;
}

function merge(
  grid1: boolean[],
  grid2: boolean[],
  size1: Point,
  size2: Point,
): {
  grid: boolean[];
  size: Point;
} {
  let bigGrid, smallGrid, bigSize, smallSize;
  if (grid1.length > grid2.length) {
    bigGrid = grid1;
    smallGrid = grid2;
    bigSize = size1;
    smallSize = size2;
  } else {
    bigGrid = grid2;
    smallGrid = grid1;
    bigSize = size2;
    smallSize = size1;
  }

  let grid = bigGrid.slice();

  for (let y = 0; y < smallSize.y; y++) {
    for (let x = 0; x < smallSize.x; x++) {
      let bigX = x + (bigSize.x - smallSize.x);
      let bigY = y + (bigSize.y - smallSize.y);
      const bigI = bigX + bigY * bigSize.x;
      grid[bigI] = bigGrid[bigI] || smallGrid[x + y * smallSize.x];
    }
  }

  return {grid, size: bigSize};
}

function countGrid(grid: boolean[]): number {
  return grid.reduce((acc, value) => (value ? acc + 1 : acc), 0);
}
