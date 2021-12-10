import {decodeText, chunkLines} from '../iter.ts';

const stdin = chunkLines(decodeText(Deno.iter(Deno.stdin)));

const openChars = ['(', '[', '{', '<'];
const closeChars = [')', ']', '}', '>'];

const errorScores = new Map([
  [')', 3],
  [']', 57],
  ['}', 1197],
  ['>', 25137],
]);

const completionScores = new Map(
  openChars.map((char, index) => [char, index + 1]),
);

const openSet = new Set(openChars);

const closeToOpen = new Map(
  closeChars.map((char, index) => [char, openChars[index]]),
);

let errorScore = 0;
const incompleteScores = [];
for await (const line of stdin) {
  const scope = [];

  try {
    for (const char of line) {
      if (openSet.has(char)) {
        scope.push(char);
      } else if (scope.at(-1) === closeToOpen.get(char)) {
        scope.pop();
      } else {
        errorScore += errorScores.get(char) ?? 0;
        throw new Error('oops');
      }
    }

    let score = 0;
    for (const char of scope.slice().reverse()) {
      score = 5 * score + (completionScores.get(char) ?? 0);
    }
    incompleteScores.push(score);
  } catch (_error) {
    // pass
  }
}

incompleteScores.sort((a, b) => b - a);

const finalScore = incompleteScores[Math.floor(incompleteScores.length / 2)];

console.log('errorScore', errorScore);
console.log('completionScore', finalScore);
