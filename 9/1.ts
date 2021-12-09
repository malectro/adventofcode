const source = 'input';
const file = await Deno.readTextFile(new URL(source, import.meta.url));

const heightmap = file.trim().split('\n').map(
  line => line.trim().split('').map(text => parseInt(text)),
);

let totalRisk = 0;
for (const [y, row] of heightmap.entries()) {
  for (const [x, value] of row.entries()) {
    const isLow = [[0, -1], [1, 0], [0, 1], [-1, 0]].every(
      ([aX, aY]) => {
        const otherValue = heightmap[y + aY]?.[x + aX];
        return otherValue == null || otherValue > value;
      }
    );

    if (isLow) {
      totalRisk += 1 + value;
    }
  }
}

console.log('total risk', totalRisk);
