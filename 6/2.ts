const days = 256;
const source = 'input';

const file = await Deno.readTextFile(new URL(source, import.meta.url));

const fish = file.trim().split(',').map(text => parseInt(text));

let queue = fill(9, 0);
for (const time of fish) {
  queue[time]++;
}

for (let i = 0; i < days; i++) {
  let born = queue.shift() ?? 0;
  queue.push(born);
  queue[6] += born;
}

const total = sum(queue);

console.log('fish count', total);

function fill<T>(size: number, value: T): T[] {
  let result = [];
  for (let i = 0; i < size; i++) {
    result.push(value);
  }
  return result;
} 

function sum(numbers: number[]): number {
  return numbers.reduce((total, value) => value + total, 0);
}
