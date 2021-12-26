import {chunkLines, decodeText, map} from '../iter.ts';

const file = await Deno.open(new URL('input', import.meta.url));

let iter = Deno.iter(file);

let textIter = decodeText(iter);
textIter = chunkLines(textIter);

const numbers = [];
for await (const data of textIter) {
  numbers.push(data);
}

const length = 12;

const oxygenList = filterNumbers(numbers, (count) => (count >= 0 ? '1' : '0'));
const co2List = filterNumbers(numbers, (count) => (count < 0 ? '1' : '0'));

console.log('oxygen', oxygenList);
console.log('co2', co2List);
console.log('final', parseInt(oxygenList[0], 2) * parseInt(co2List[0], 2));

function filterNumbers(
  numbers: string[],
  getDesiredBit: (count: number) => string,
): string[] {
  let list = numbers;
  for (let i = 0; i < length; i++) {
    let newList = [];
    const desiredBit = getDesiredBit(getCount(list, i));
    newList = list.filter((data) => data[i] === desiredBit);
    if (newList.length === 0) {
      break;
    }
    list = newList;
  }
  return list;
}

function getCount(numbers: string[], index: number): number {
  let count = 0;
  for (const data of numbers) {
    if (data[index] === '0') {
      count -= 1;
    } else {
      count += 1;
    }
  }
  return count;
}
