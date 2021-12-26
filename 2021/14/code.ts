import {
  decodeText,
  chunkLines,
  filter,
  takeWhile,
  map,
  mapNonNullable,
  toArray,
} from '../iter.ts';

const stdin = chunkLines(decodeText(Deno.iter(Deno.stdin)));

const templates = await toArray(takeWhile(stdin, (line) => line));

let template = templates[0];

const rules = new Map();
for await (const line of stdin) {
  const components = line.trim().split(' -> ');
  rules.set(components[0], components[1]);
}

let pairCounts = new Map();
for (const i of range(0, template.length - 1)) {
  const pair = template.slice(i, i + 2);
  pairCounts.set(pair, (pairCounts.get(pair) ?? 0) + 1);
}

for (const i of range(0, 40)) {
  const newPairCounts = new Map();

  for (const [pair, count] of pairCounts) {
    const rule = rules.get(pair);

    if (rule) {
      newPairCounts.set(
        pair[0] + rule,
        (newPairCounts.get(pair[0] + rule) ?? 0) + count,
      );
      newPairCounts.set(
        rule + pair[1],
        (newPairCounts.get(rule + pair[1]) ?? 0) + count,
      );
    }
  }

  pairCounts = newPairCounts;
}

let letterCounts = new Map();
for (const [pair, count] of pairCounts) {
  letterCounts.set(pair[1], (letterCounts.get(pair[1]) ?? 0) + count);
}
letterCounts.set(template[0], (letterCounts.get(template[0]) ?? 0) + 1);

const [min, max] = [...letterCounts.values()].reduce(
  ([min, max], count) => [Math.min(count, min), Math.max(count, max)],
  [Infinity, 0],
);

console.log('final value', max - min);

function* range(from: number, to: number) {
  for (let i = from; i < to; i++) {
    yield i;
  }
}
