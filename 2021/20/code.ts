import {
  decodeText,
  chunkLines,
  range,
  toArray,
} from '../iter.ts';

const lines = await toArray(chunkLines(decodeText(Deno.iter(Deno.stdin))));

const decoder = lines.shift() ?? '';
lines.shift();

//console.log('decoder', decoder);

let image = lines.map((line) =>
  Array.from(line).map((char) => (char === '#' ? 1 : 0)),
);

//console.log('image', '\n', serializeImage(image), '\n');

let zeroSpace = 0;
for (const _ of range(0, 50)) {
  const newImage = Array(image.length + 2)
    .fill(null)
    .map((_) => Array(image[0].length + 2).fill(0));

  const offsets = [-1, 0, 1];
  for (const [i, row] of newImage.entries()) {
    for (const j of row.keys()) {
      let key = 0;
      for (const k of offsets) {
        for (const l of offsets) {
          key = (key << 1) + (image[i + k - 1]?.[j + l - 1] ?? zeroSpace);
        }
      }
      row[j] = decoder[key] === '#' ? 1 : 0;
    }
  }

  //console.log('zoom', '\n', serializeImage(newImage), '\n');

  image = newImage;

  if (zeroSpace === 0 && decoder[0] === '#') {
    zeroSpace = 1;
  } else if (zeroSpace === 1 && decoder[511] === '.') {
    zeroSpace = 0;
  }
}

console.log(
  'count',
  image.reduce(
    (count, row) => row.reduce((count, pixel) => count + pixel, count),
    0,
  ),
);

function serializeImage(image: number[][]): string {
  return image.map((row) => row.join('')).join('\n');
}
