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

//console.log('scanners', scanners);

const tree = buildTree(
  scanners.map((beacons) => ({beacons, diffs: getDifferenceMatrix(beacons)})),
);

const beacons = mergeTree(tree);
console.log('beacon count', beacons.length);

const scannerPositions = getScannerPositions(tree);

let max = 0;
for (const point1 of scannerPositions) {
  for (const point2 of scannerPositions) {
    const diff = Pt.abs(Pt.difference(point1, point2));
    max = Math.max(max, diff.x + diff.y + diff.z);
  }
}

console.log('max distance', max);

/*
let beacons = scanners[0];

let scannersLeft = Array.from(scanners.entries()).slice(1);
while (scannersLeft.length > 0) {
  //console.log('scanners left', scannersLeft.length);
  const missedScanners: Array<[number, Point[]]> = [];
  for (const [name, scanner] of scannersLeft) {
    const differences = getDifferenceMatrix(beacons);
    const scannerDiffs = getDifferenceMatrix(scanner);

    const transform = getTransform(beacons, differences, scanner, scannerDiffs);

    if (transform) {
      beacons = mergeScanners(beacons, scanner.map(transform));
      console.log(
        `scanner ${name} position`,
        transform(Pt.make()),
        beacons.length,
      );
      console.log(
        beacons.slice().sort((a, b) => {
          if (a.x !== b.x) {
            return a.x - b.x;
          }
          if (a.y !== b.y) {
            return a.y - b.y;
          }
          return a.z - b.z;
        }),
      );
    } else {
      missedScanners.push([name, scanner]);
    }
  }
  scannersLeft = missedScanners;
}
*/

//console.log('final size', beacons.length);

/*
const scanner2 = scanners[1];

const s1Differences = getDifferenceMatrix(scanner1);

console.log('difference', s1Differences[0]);

const s2Differences = getDifferenceMatrix(scanner2);

let result;

let pointMap;
let signPoint;
for (const [i, beaconA] of scanner2.entries()) {
  let count = 0;
  const currentMap = [];
  for (const [j, beaconB] of scanner2.entries()) {
    const difference = Pt.difference(beaconA, beaconB);
    const index = s1Differences[0].findIndex((beacon) =>
      Pt.areEqual(Pt.abs(beacon), Pt.abs(difference)),
    );
    if (index >= 0) {
      currentMap[j] = {
        index,
        difference,
      };
      count++;
    } else {
      currentMap[j] = null;
    }
    //difference[i][j] = Pt.difference(beaconA, beaconB);
  }
  if (count > 11) {
    pointMap = currentMap;
    console.log('found 12 similar points', i);
  }
}

if (pointMap) {
  const map = new Map();
  for (const [index, item] of pointMap.entries()) {
    if (item != null) {
      const {index: mappedIndex, difference} = item;
      const s1Difference = s1Differences[0][mappedIndex];
      const sign = {
        x: Math.sign(s1Difference.x) * Math.sign(difference.x),
        y: Math.sign(s1Difference.y) * Math.sign(difference.y),
        z: Math.sign(s1Difference.z) * Math.sign(difference.z),
      };
      map.set(scanner2[index], {point: scanner1[mappedIndex], sign});
    }
  }

  const diffs = Array.from(map).map(([point1, {point: point2, sign}]) =>
    Pt.difference(point1, Pt.multiply(point2, sign)),
  );

  console.log('point map', map, diffs);
}

const transform = getTransform(
  scanner1,
  s1Differences,
  scanner2,
  s2Differences,
);

if (transform) {
  const merged = mergeScanners(scanner1, scanner2.map(transform));
  console.log('test', scanner1, scanner2.map(transform));

  console.log('merged', scanner1.length, merged.length);
}
*/
interface Scanner {
  beacons: Point[];
  diffs: Point[][];
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

  /*
  let entry;
  while ((entry = leftOver.shift())) {
    const [name, scanner] = entry;
    console.log('trying', name);
    const wasInserted = insert(tree, scanner);

    if (!wasInserted) {
      console.log('success!');
      leftOver.push(entry);
    }
  }
  */

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

  //return tree.children.some((child) => insert(child, scanner));
}

function mergeTree(tree: Tree): Point[] {
  return tree.children.reduce(
    (beacons, tree) => mergeScanners(beacons, mergeTree(tree).map(
      beacon => tree.transform ? tree.transform(beacon) : beacon
    )),
    tree.scanner.beacons,
  );
}

function getScannerPositions(tree: Tree): Point[] {
  const points = [Pt.make()];

  for (const child of tree.children) {
    points.push(
      ...getScannerPositions(child).map(
        point => child.transform ? child.transform(point) : point,
      ),
    );
  }

  return points;
}

function getDifferenceMatrix(scanner: Point[]): Point[][] {
  const s1Differences = Array(scanner.length)
    .fill(null)
    .map((_) => Array(scanner.length).fill(Pt.make()));

  for (const [i, beaconA] of scanner.entries()) {
    for (const [j, beaconB] of scanner.entries()) {
      s1Differences[i][j] = Pt.difference(beaconA, beaconB);
    }
  }

  return s1Differences;
}

function getTransform(
  scanner1: Point[],
  differences1: Point[][],
  scanner2: Point[],
  differences2: Point[][],
) {
  for (const diffRow0 of differences1) {
    for (const [i, row] of differences2.entries()) {
      for (const format of Pt.formats) {
        for (const sign of Pt.pointSigns) {
          let count = 0;
          let currentMap = [];
          for (const [j, diff2] of row.entries()) {
            if (i !== j) {
              const index = diffRow0.findIndex((diff1) =>
                Pt.areEqual(
                  Pt.multiply(Pt.reformat(diff2, format), sign),
                  diff1,
                ),
              );
              if (index >= 0) {
                count++;
                currentMap[j] = {
                  index,
                  format,
                  sign,
                  diff1: diffRow0[index],
                  diff2,
                };
              } else {
                currentMap[j] = null;
              }
            }
          }
          //console.log('row count', i, count);
          if (count > 10) {
            for (const [i, item] of currentMap.entries()) {
              if (item) {
                const scannerDiff = Pt.difference(
                  scanner1[item.index],
                  Pt.multiply(Pt.reformat(scanner2[i], format), sign),
                );

                //console.log(sign, scannerDiff);

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