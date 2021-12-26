import {slidingWindows} from 'https://deno.land/std@0.119.0/collections/sliding_windows.ts';
import {decodeText, chunkLines, map, range, clamp, toArray} from '../iter.ts';

const lines = chunkLines(decodeText(Deno.iter(Deno.stdin)));

const instructions = await toArray(
  map(
    map(lines, (line) => {
      const match =
        /^(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)$/.exec(
          line,
        );
      if (!match) {
        throw new Error('Invalid instruction');
      }
      return match;
    }),
    (match) => ({
      action: match[1],
      from: {
        x: parseInt(match[2]),
        y: parseInt(match[4]),
        z: parseInt(match[6]),
      },
      to: {
        x: parseInt(match[3]),
        y: parseInt(match[5]),
        z: parseInt(match[7]),
      },
    }),
  ),
);

// because the ranges in the instructions are sometimes backwards
for (const {from, to} of instructions) {
  if (from.x > to.x) {
    [from.x, to.x] = [to.x, from.x];
  }
  if (from.y > to.y) {
    [from.y, to.y] = [to.y, from.y];
  }
  if (from.z > to.z) {
    [from.z, to.z] = [to.z, from.z];
  }
}

// part 1
const cubes = Array(101)
  .fill(null)
  .map((_) =>
    Array(101)
      .fill(null)
      .map((_) => Array(101).fill(false)),
  );

for (const {action, from, to} of instructions) {
  const value = action === 'on' ? true : false;
  for (const x of range(clamp(from.x + 50, 0, 100), clamp(to.x + 51, 0, 100))) {
    for (const y of range(
      clamp(from.y + 50, 0, 100),
      clamp(to.y + 51, 0, 100),
    )) {
      for (const z of range(
        clamp(from.z + 50, 0, 100),
        clamp(to.z + 51, 0, 100),
      )) {
        cubes[x][y][z] = value;
      }
    }
  }
}

const count = cubes.reduce(
  (count, plane) =>
    plane.reduce(
      (count, row) =>
        row.reduce((count, status) => (status ? 1 + count : count), count),
      count,
    ),
  0,
);

console.log('count', count);

// part 2
let shapes: Shape[] = [];

for (const {action, from, to} of instructions) {
  const newShape = {from, to};

  const newShapes = [];
  let intShape;
  for (const shape of shapes) {
    intShape = intersection(shape, newShape);

    if (intShape) {
      newShapes.push(...split(shape, intShape));
    } else {
      newShapes.push(shape);
    }
  }

  if (action === 'on') {
    newShapes.push(newShape);
  }

  shapes = newShapes;
}

console.log('total shapes', shapes.length);
console.log(
  'total volume',
  shapes.reduce(
    (volume, shape) =>
      (shape.to.x + 1 - shape.from.x) *
        (shape.to.y + 1 - shape.from.y) *
        (shape.to.z + 1 - shape.from.z) +
      volume,
    0,
  ),
);

interface Point {
  x: number;
  y: number;
  z: number;
}

interface Shape {
  from: Point;
  to: Point;
}

function intersection(shape1: Shape, shape2: Shape): Shape | null {
  const shape = {
    from: {
      x: Math.max(shape1.from.x, shape2.from.x),
      y: Math.max(shape1.from.y, shape2.from.y),
      z: Math.max(shape1.from.z, shape2.from.z),
    },
    to: {
      x: Math.min(shape1.to.x, shape2.to.x),
      y: Math.min(shape1.to.y, shape2.to.y),
      z: Math.min(shape1.to.z, shape2.to.z),
    },
  };

  if (
    shape.from.x > shape.to.x ||
    shape.from.y > shape.to.y ||
    shape.from.z > shape.to.z
  ) {
    return null;
  }

  return shape;
}

// TODO: can probably split into fewer shapes
function split(shape1: Shape, shape2: Shape): Shape[] {
  const xList = slidingWindows(
    [shape1.from.x, shape2.from.x, shape2.to.x + 1, shape1.to.x + 1],
    2,
  );

  const yList = slidingWindows(
    [shape1.from.y, shape2.from.y, shape2.to.y + 1, shape1.to.y + 1],
    2,
  );

  const zList = slidingWindows(
    [shape1.from.z, shape2.from.z, shape2.to.z + 1, shape1.to.z + 1],
    2,
  );

  const shapes = [];
  for (let [fromX, toX] of xList) {
    toX -= 1;
    for (let [fromY, toY] of yList) {
      toY -= 1;
      for (let [fromZ, toZ] of zList) {
        toZ -= 1;
        if (
          (fromX !== shape2.from.x ||
            fromY !== shape2.from.y ||
            fromZ !== shape2.from.z) &&
          toX >= fromX &&
          toY >= fromY &&
          toZ >= fromZ
        ) {
          shapes.push({
            from: {
              x: fromX,
              y: fromY,
              z: fromZ,
            },
            to: {
              x: toX,
              y: toY,
              z: toZ,
            },
          });
        }
      }
    }
  }
  return shapes;
}
