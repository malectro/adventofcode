import {chunkLines, decodeText, map} from '../iter.ts';

const file = await Deno.open(new URL('input', import.meta.url));

let iter = Deno.iter(file);

let textIter = decodeText(iter);
textIter = chunkLines(textIter);

const counts = fill([], 0, 12);
for await (const data of textIter) {
  for (let i = 0; i < 12; i++) {
    if (data[i] === '0') {
      counts[i] -= 1;
    } else {
      counts[i] += 1;
    }
  }
}

let gamma = calcWeirdValue(counts.map((value) => (value > 0 ? '1' : '0')));

let epsilon = calcWeirdValue(counts.map((value) => (value > 0 ? '0' : '1')));

console.log('counts', counts);
console.log(
  'gamma',
  counts.map((value) => (value > 0 ? '1' : '0')).join(''),
  gamma,
),
  console.log(
    'epsilon',
    counts.map((value) => (value < 0 ? '1' : '0')).join(''),
    epsilon,
  ),
  console.log('final', gamma * epsilon);

function calcWeirdValue(bits: string[]): number {
  return parseInt(bits.join(''), 2);
}

function fill<T>(array: Array<T>, value: T, size: number): Array<T> {
  for (let i = 0; i < size; i++) {
    array[i] = value;
  }
  return array;
}
