import {range} from '../iter.ts';

//const data = 'target area: x=20..30, y=-10..-5';
const data = 'x=57..116, y=-198..-148';

const match = data.match(
  /x=(?<xF>-?\d+)..(?<xT>-?\d+), y=(?<yF>-?\d+)..(?<yT>-?\d+)$/,
);

if (!match || !match.groups) {
  throw new Error('Invalid input data');
}

const {xF, xT, yF, yT} = match.groups;
const target = {
  origin: {
    x: parseInt(xF),
    y: parseInt(yF),
  },
  end: {
    x: parseInt(xT),
    y: parseInt(yT),
  },
};
console.log('target', target);

console.log('test flight', testFlight(target, {x: 6, y: 9}));

let successes = [];
// brute force commented here
/*
for (let x of range(0, target.end.x + 1)) {
  for (let y of range(target.origin.y, 1_000)) {
    const [isGood, maxY] = testFlight(target, {x, y});
    if (isGood) {
      successes.push({
        maxY,
        x,
        y,
      });
    }
  }
}
*/

const validXs = [...range(0, target.end.x + 1)].filter(x => {
  let position = 0;
  while (position <= target.end.x) {
    console.log(x, position);
    if (position >= target.origin.x) {
      return true;
    }

    position += x;
    x -= Math.sign(x);

    if (x === 0) {
      return false;
    }
  }
  return false;
});

const validYs = [...range(target.origin.y, 10_000)].filter(y => {
  let position = 0;
  while (position >= target.origin.y) {
    if (position <= target.end.y) {
      return true;
    }

    position += y;
    y -= 1;
  }
  return false;
});

for (const x of validXs) {
  for (const y of validYs) {
    const [isGood, maxY] = testFlight(target, {x, y});
    if (isGood) {
      successes.push({
        maxY,
        x,
        y,
      });
    }
  }
}

const maxY = successes.reduce((acc, {maxY}) => Math.max(maxY, acc), -Infinity);

console.log('maxY', maxY);
console.log('total', successes.length);

interface Point {
  x: number;
  y: number;
}

interface Target {
  origin: Point;
  end: Point;
}

function testFlight(target: Target, velocity: Point): [boolean, number] {
  let maxY = 0;
  let position = {x: 0, y: 0};

  while (position.y >= target.origin.y) {
    position = addPoints(position, velocity);
    maxY = Math.max(maxY, position.y);

    if (targetContains(target, position)) {
      return [true, maxY];
    }

    velocity.x -= Math.sign(velocity.x);
    velocity.y -= 1;
  }

  return [false, maxY];
}

function addPoints(point1: Point, point2: Point): Point {
  return {
    x: point1.x + point2.x,
    y: point1.y + point2.y,
  };
}

function targetContains(target: Target, point: Point): boolean {
  return (
    point.x >= target.origin.x &&
    point.x <= target.end.x &&
    point.y >= target.origin.y &&
    point.y <= target.end.y
  );
}
