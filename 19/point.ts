export interface Point {
  x: number;
  y: number;
  z: number;
}

export function make(x = 0, y = 0, z = 0): Point {
  return {x, y, z};
}

export function difference(point1: Point, point2: Point): Point {
  return {
    x: point2.x - point1.x,
    y: point2.y - point1.y,
    z: point2.z - point1.z,
  };
}

export function multiply(point1: Point, point2: Point): Point {
  return {
    x: point2.x * point1.x,
    y: point2.y * point1.y,
    z: point2.z * point1.z,
  };
}

export function abs(point: Point): Point {
  return {
    x: Math.abs(point.x),
    y: Math.abs(point.y),
    z: Math.abs(point.z),
  };
}

export function sign(point: Point): Point {
  return {
    x: Math.sign(point.x),
    y: Math.sign(point.y),
    z: Math.sign(point.z),
  };
}

export function areEqual(point1: Point, point2: Point): boolean {
  return (
    point1.x === point2.x && point1.y === point2.y && point1.z === point2.z
  );
}

type Axis = 'x' | 'y' | 'z';
type PointFormat = {
  x: Axis;
  y: Axis;
  z: Axis;
};
export function reformat(point: Point, map: PointFormat): Point {
  return {
    x: point[map.x],
    y: point[map.y],
    z: point[map.z],
  };
}

const axes: Array<Axis> = ['x', 'y', 'z'];
const formats: PointFormat[] = [];
for (const x of axes) {
  for (const y of axes) {
    if (y !== x) {
      for (const z of axes) {
        if (z !== x && z !== y) {
          formats.push({x, y, z});
        }
      }
    }
  }
}
export {formats};
