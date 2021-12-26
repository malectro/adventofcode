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
import * as Pt from './point.ts';
type Point = Pt.Point;

const lines = chunkLines(decodeText(Deno.iter(Deno.stdin)));

const scanners = [];
let scanner = [];
for await (let line of lines) {
  if (line.includes('scanner')) {
    scanner = [];
    continue;
  }

  if (line.trim() === '') {
    scanners.push(scanner);
    continue;
  }

  const [x, y, z] = line
    .trim()
    .split(',')
    .map((n) => parseInt(n));
  scanner.push({x, y, z});
}
scanners.push(scanner);

const tree = buildTree(
  scanners.map((beacons) => ({beacons, diffs: getDifferenceMatrix(beacons)})),
);

const beacons = mergeTree(tree);
console.log('beacon count', beacons.length);

const scannerPositions = getScannerPositions(tree);

let max = 0;
for (const point1 of scannerPositions) {
  for (const point2 of scannerPositions) {
    max = Math.max(max, Pt.manhattan(point1, point2));
  }
}

console.log('max distance', max);

interface Scanner {
  beacons: Point[];
  diffs: number[][];
}

interface Tree {
  scanner: Scanner;
  children: Tree[];
  transform?: (point: Point) => Point;
}

function buildTree(scanners: Scanner[]): Tree {
  const first = scanners[0];

  let leftOver = Array.from(scanners.entries()).slice(1);

  const tree = {
    scanner: first,
    children: [],
  };

  const toTry: Tree[] = [tree];

  let node;
  while ((node = toTry.shift())) {
    const newLeftOver = [];
    for (const entry of leftOver) {
      const [name, scanner] = entry;
      console.log('trying', name);

      const newNode = insert(node, scanner);
      if (newNode) {
        console.log('success!');
        toTry.push(newNode);
      } else {
        newLeftOver.push(entry);
      }
    }
    leftOver = newLeftOver;
  }

  return tree;
}

function insert(tree: Tree, scanner: Scanner): Tree | void {
  const first = tree.scanner;
  const transform = getTransform(
    first.beacons,
    first.diffs,
    scanner.beacons,
    scanner.diffs,
  );
  if (transform) {
    const newNode = {
      scanner,
      transform,
      children: [],
    };
    tree.children.push(newNode);
    return newNode;
  }
}

function mergeTree(tree: Tree): Point[] {
  return tree.children.reduce(
    (beacons, tree) =>
      mergeScanners(
        beacons,
        mergeTree(tree).map((beacon) =>
          tree.transform ? tree.transform(beacon) : beacon,
        ),
      ),
    tree.scanner.beacons,
  );
}

function getScannerPositions(tree: Tree): Point[] {
  const points = [Pt.make()];

  for (const child of tree.children) {
    points.push(
      ...getScannerPositions(child).map((point) =>
        child.transform ? child.transform(point) : point,
      ),
    );
  }

  return points;
}

function getDifferenceMatrix(scanner: Point[]): number[][] {
  const s1Differences = Array(scanner.length)
    .fill(null)
    .map((_) => Array(scanner.length).fill(Pt.make()));

  for (const [i, beaconA] of scanner.entries()) {
    for (const [j, beaconB] of scanner.entries()) {
      s1Differences[i][j] = Pt.manhattan(beaconA, beaconB);
    }
  }

  return s1Differences;
}

function getTransform(
  scanner1: Point[],
  differences1: number[][],
  scanner2: Point[],
  differences2: number[][],
) {
  for (const [index1, diffRow0] of differences1.entries()) {
    for (const [i, row] of differences2.entries()) {
      let count = 0;
      let currentMap = [];
      for (const [j, diff2] of row.entries()) {
        if (i !== j) {
          const index2 = diffRow0.findIndex((diff1) => diff1 === diff2);
          if (index2 >= 0) {
            count++;
            currentMap[j] = {
              index1,
              index2,
              i,
              j,
              diff1: diffRow0[index2],
              diff2,
              point1: scanner1[index2],
              point2: scanner2[j],
            };
          } else {
            currentMap[j] = null;
          }
        }
      }
      if (count > 10) {
        for (const [i, item] of currentMap.entries()) {
          if (item) {
            const diff1 = Pt.difference(
              scanner1[item.index1],
              scanner1[item.index2],
            );
            const diff2 = Pt.difference(scanner2[item.i], scanner2[item.j]);

            const stuff = resolveFormat(diff1, diff2);

            if (stuff) {
              const [format, sign] = stuff;
              const scannerDiff = Pt.difference(
                scanner1[item.index2],
                Pt.multiply(Pt.reformat(scanner2[i], format), sign),
              );

              return (point: Point) => {
                return Pt.difference(
                  scannerDiff,
                  Pt.multiply(Pt.reformat(point, format), sign),
                );
              };
            }
          }
        }
      }
    }
  }
}

function mergeScanners(scanner1: Point[], scanner2: Point[]): Point[] {
  const result = scanner1.slice();

  for (const point2 of scanner2) {
    if (!scanner1.find((point1) => Pt.areEqual(point1, point2))) {
      result.push(point2);
    }
  }

  return result;
}

function resolveFormat(
  diff1: Point,
  diff2: Point,
): [Pt.PointFormat, Point] | void {
  for (const format of Pt.formats) {
    for (const sign of Pt.pointSigns) {
      if (Pt.areEqual(Pt.multiply(Pt.reformat(diff2, format), sign), diff1)) {
        return [format, sign];
      }
    }
  }
}
