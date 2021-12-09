const source = 'input';
const file = await Deno.readTextFile(new URL(source, import.meta.url));

const directions = [
  [0, -1],
  [1, 0],
  [0, 1],
  [-1, 0],
].map(([x, y]) => ({x, y}));

const heightmap = file
  .trim()
  .split('\n')
  .map((line) =>
    line
      .trim()
      .split('')
      .map((text) => parseInt(text)),
  );

const lowPoints = [];

let totalRisk = 0;
for (const [y, row] of heightmap.entries()) {
  for (const [x, value] of row.entries()) {
    const isLow = directions.every((direction) => {
      const otherValue = heightmap[y + direction.y]?.[x + direction.x];
      return otherValue == null || otherValue > value;
    });

    if (isLow) {
      lowPoints.push({x, y});
      totalRisk += 1 + value;
    }
  }
}

console.log('total risk', totalRisk);

const basinMap = heightmap.map((row) => row.slice().fill(-1));

console.log('heightmap');
console.log(serializeMap(heightmap));

let basinSizes = [];
for (const [index, lowPoint] of lowPoints.entries()) {
  basinSizes.push(
    paintBasin(basinMap, index, lowPoint)
  );
}

console.log('basinMap\n', serializeMap(basinMap));
console.log('basins', basinSizes);

let top3: number[] = [];
for (const size of basinSizes) {
  top3.push(size);
  top3.sort((a, b) => b - a);
  top3 = top3.slice(0, 3);
}

console.log('top3', top3);
console.log('final value', top3.reduce((acc, size) => size * acc, 1));

type Point = {x: number, y: number};

function paintBasin(basinMap: number[][], basinId: number, point: Point): number {
  const {x, y} = point;
  const currentBasin = basinMap[y]?.[x];

  //console.log('currentBasin', currentBasin, heightmap[y]?.[x]);

  if (currentBasin !== -1 || heightmap[y][x] === 9) {
    return 0;
  }

  let size = 1;

  basinMap[y][x] = basinId;

  for (const direction of directions) {
    size += paintBasin(basinMap, basinId, addPoint(point, direction));
  }

  return size;
}

function addPoint(point1: Point, point2: Point): Point {
  return {
    x: point1.x + point2.x,
    y: point1.y + point2.y,
  };
}

function serializeMap(map: number[][]) {
  return map.map(row => row.map(v => v === -1 ? 'x' : v).join('')).join('\n')
}
