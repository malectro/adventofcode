const source = 'input';
const file = await Deno.readTextFile(new URL(source, import.meta.url));

const crabs = file
  .trim()
  .split(',')
  .map((text) => parseInt(text));

console.log('average', sum(crabs) / crabs.length);

crabs.sort((a, b) => a - b);

console.log(
  'total distance',
  totalDistance(crabs, crabs[Math.floor(crabs.length / 2)]),
);

function sum(numbers: number[]): number {
  return numbers.reduce((total, value) => value + total, 0);
}

function totalDistance(crabs: number[], position: number): number {
  return crabs.reduce(
    (distance, value) => distance + Math.abs(position - value),
    0,
  );
}
