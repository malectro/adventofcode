import {decodeText, chunkLines, map} from '../iter.ts';

const file = await Deno.open(new URL('input', import.meta.url));

let iter = Deno.iter(file);

let textIter = decodeText(iter);
textIter = chunkLines(textIter);
let numbers = map(textIter, parseInt);

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
