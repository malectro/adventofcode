const source = 'example';
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

for (const [index, lowPoint] of lowPoints.entries()) {
  paintBasin(basinMap, index, lowPoint);
}

console.log('basinMap\n', serializeMap(basinMap));

type Point = {x: number, y: number};

function paintBasin(basinMap: number[][], basinId: number, point: Point) {
  const {x, y} = point;
  const currentBasin = basinMap[y]?.[x];

  //console.log('currentBasin', currentBasin, heightmap[y]?.[x]);

  if (currentBasin !== -1 || heightmap[y][x] === 9) {
    return;
  }

  basinMap[y][x] = basinId;

  for (const direction of directions) {
    paintBasin(basinMap, basinId, addPoint(point, direction));
  }
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
