const data = 'target area: x=20..30, y=-10..-5';

const match = data.match(/ x=(?<xF>-?\d+)..(?<xT>-?\d+), y=(?<yF>-?\d+)..(?<yT>-?\d+)$/);

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

interface Point {
  x: number;
  y: number;
}

interface Target {
  origin: Point;
  end: Point;
};

function testFlight(target: Target, velocity: Point) {
  let maxY = 0;
  let position = {x: 0, y: 0};

  while (position.x < target.end.x && position.y > target.end.y) {
    position = addPoints(position, velocity);
    maxY = Math.max(maxY, position.y);

    if (targetContains(target, position)) {
      return maxY;
    }

    velocity.x -= Math.sign(velocity.x);
    velocity.y -= 1;
  }
}

function addPoints(point1: Point, point2: Point): Point {
  return {
    x: point1.x + point2.x,
    y: point1.y + point2.y,
  };
}

function targetContains(target: Target, point: Point): boolean {
  return point.x >= target.origin.x && point.x <= target.end.x && point.y >= target.origin.y && point.y <= target.end.y;
}
