import {decodeText, chunkLines, map, rollingWindow} from '../iter.ts';

const file = await Deno.open(new URL('input', import.meta.url));

let iter = Deno.iter(file);

let textIter = decodeText(iter);
textIter = chunkLines(textIter);
let numbers = map(textIter, parseInt);
let windows = rollingWindow(numbers, 3);
numbers = map(windows, sum);

let count = 0;
let prev = (await numbers.next()).value;
for await (const current of numbers) {
  if (current > prev) {
    count++;
  }
  prev = current;
}

console.log('count', count);

file.close();

function sum(array: Array<number>): number {
  let value = 0;
  for (const number of array) {
    value += number;
  }
  return value;
}
