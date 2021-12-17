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

const maxY = successes.reduce((acc, {maxY}) => Math.max(maxY, acc), -Infinity);

console.log('maxY', maxY);
console.log('total', successes.length);
//console.log('all', successes);


//console.log('test flight 2', testFlight(target, {x: 6, y: 0}));

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

/*
 triangular numbers
 x = 1, p = 1
 x = 2, p = 3
 x = 3, p = 6
 x = 4, p = 10
 p = (x * x + x) / 2
*/


/*
const exampleResult = `23,-10  25,-9   27,-5   29,-6   22,-6   21,-7   9,0     27,-7   24,-5
25,-7   26,-6   25,-5   6,8     11,-2   20,-5   29,-10  6,3     28,-7
8,0     30,-6   29,-8   20,-10  6,7     6,4     6,1     14,-4   21,-6
26,-10  7,-1    7,7     8,-1    21,-9   6,2     20,-7   30,-10  14,-3
20,-8   13,-2   7,3     28,-8   29,-9   15,-3   22,-5   26,-8   25,-8
25,-6   15,-4   9,-2    15,-2   12,-2   28,-9   12,-3   24,-6   23,-7
25,-10  7,8     11,-3   26,-7   7,1     23,-9   6,0     22,-10  27,-6
8,1     22,-8   13,-4   7,6     28,-6   11,-4   12,-4   26,-9   7,4
24,-10  23,-8   30,-8   7,0     9,-1    10,-1   26,-5   22,-9   6,5
7,5     23,-6   28,-10  10,-2   11,-1   20,-9   14,-2   29,-7   13,-3
23,-5   24,-8   27,-9   30,-7   28,-5   21,-10  7,9     6,6     21,-5
27,-10  7,2     30,-9   21,-8   22,-7   24,-9   20,-6   6,9     29,-5
8,-2    27,-8   30,-5   24,-7`;
const exampleSuccesses = exampleResult.split(/\s+/).map(
  string => string.split(','),
).map(([x, y]) => ({x: parseInt(x), y: parseInt(y)})).sort((a, b) => {
  if (a.x === b.x) {
    return a.y - b.y;
  }
  return a.x - b.x;
});
console.log('example', exampleSuccesses);
*/
