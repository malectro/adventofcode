const source = 'example';
const file = await Deno.readTextFile(new URL(source, import.meta.url));

const lines = file.split('\n');

const openChars = [
  '(',
  '[',
  '{',
  '<',
];

const closeChars = [
  ')',
  ']',
  '}',
  '>',
];

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
for (const line of lines) {
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
