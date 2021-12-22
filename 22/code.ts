import {
  decodeText,
  chunkLines,
  reChunk,
  take,
  map,
  reduce,
  range,
  toArray,
} from '../iter.ts';

const lines = chunkLines(decodeText(Deno.iter(Deno.stdin)));

const instructions = map(
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
);

//console.log('instructions', await toArray(instructions));

const cubes = Array(101)
  .fill(null)
  .map((_) =>
    Array(101)
      .fill(null)
      .map((_) => Array(101).fill(false)),
  );

for await (const {action, from, to} of instructions) {
  const value = action === 'on' ? true : false;
  for (const x of range(Math.max(0, from.x + 50), Math.min(101, to.x + 51))) {
    for (const y of range(Math.max(0, from.y + 50), Math.min(101, to.y + 51))) {
      for (const z of range(
        Math.max(0, from.z + 50),
        Math.min(101, to.z + 51),
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
