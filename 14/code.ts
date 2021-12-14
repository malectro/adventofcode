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

const templates = await toArray(
  takeWhile(stdin, (line) => line),
);

let template = templates[0];

console.log('template', template);

const rules = new Map();
for await (const line of stdin) {
  const components = line.trim().split(' -> ');
  rules.set(components[0], components[1]);
}

console.log('rules', rules);

for (const i of range(0, 10)) {
  let newTemplate = '';
  for (const j of range(0, template.length - 1)) {
    const pair = template.slice(j, j + 2);
    const insertion = rules.get(pair);

    if (insertion) {
      newTemplate += pair[0] + insertion;
    } else {
      newTemplate += pair[0];
    }
  }
  template = newTemplate + template[template.length - 1];
}

//console.log('done', template);

let letterCounts = new Map();
for (const letter of template) {
  letterCounts.set(letter, (letterCounts.get(letter) ?? 0) + 1);
}

console.log('counts', letterCounts);

const [min, max] = [...letterCounts.values()].reduce(([min, max], count) => [
  Math.min(count, min),
  Math.max(count, max),
], [Infinity, 0]);

console.log('final value', max - min);

function *range(from: number, to: number) {
  for (let i = from; i < to; i++) {
    yield i;
  }
}
