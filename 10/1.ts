import {decodeText, chunkLines} from '../iter.ts';

const stdin = chunkLines(decodeText(Deno.iter(Deno.stdin)));

const openChars = ['(', '[', '{', '<'];

const closeChars = [')', ']', '}', '>'];

const scores = new Map([
  [')', 3],
  [']', 57],
  ['}', 1197],
  ['>', 25137],
]);

const openSet = new Set(openChars);
const closeSet = new Set(closeChars);

const closeToOpen = closeChars.reduce((map, char, index) => {
  map.set(char, openChars[index]);
  return map;
}, new Map());

let score = 0;
for await (const line of stdin) {
  const scope = [];

  for (const char of line) {
    if (openSet.has(char)) {
      scope.push(char);
    } else if (scope.at(-1) === closeToOpen.get(char)) {
      scope.pop();
    } else {
      score += scores.get(char) ?? 0;
      break;
    }
  }
}

console.log('score', score);
