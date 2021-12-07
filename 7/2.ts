const source = 'input';
const file = await Deno.readTextFile(new URL(source, import.meta.url));

const crabs = file
  .trim()
  .split(',')
  .map((text) => parseInt(text));

crabs.sort((a, b) => a - b);

const median = Math.floor(crabs.length / 2);
console.log(
  'fuel cost from median',
  totalDistance(crabs, crabs[median]),
);

const average = Math.floor(sum(crabs) / crabs.length);
console.log(
  'fuel cost from average',
  totalDistance(crabs, average),
);

function sum(numbers: number[]): number {
  return numbers.reduce((total, value) => value + total, 0);
}

function totalDistance(crabs: number[], position: number): number {
  return crabs.reduce(
    (distance, value) => {
      const currentDistance = Math.abs(position - value);
      return distance + (currentDistance * currentDistance + currentDistance) / 2;
    },
    0,
  );
}
