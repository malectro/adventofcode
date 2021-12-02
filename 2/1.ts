import {
  decodeText,
  chunkLines,
  map,
} from '../iter.ts';

const file = await Deno.open(new URL('input', import.meta.url));

let iter = Deno.iter(file);

let textIter = decodeText(iter);
textIter = chunkLines(textIter);

let commands = map(textIter, (string) => string.split(' '));

let distance = 0;
let depth = 0;
for await (const [command, valueString] of commands) {
  const value = parseInt(valueString);
  if (command === 'forward') {
    distance += value;
  } else if (command === 'down') {
    depth += value;
  } else if (command === 'up') {
    depth -= value;
  }
}

console.log('result', distance * depth);

file.close();
