import {
  decodeText,
  chunkLines,
  map,
  range,
  clamp,
  toArray,
  takeWhile,
} from '../iter.ts';

const lines = await toArray(chunkLines(decodeText(Deno.iter(Deno.stdin))));

const program = [];
let set: string[] = [];
for (const [i, line] of lines.entries()) {
  if (line === 'inp w' && set.length > 0) {
    program.push(set);
    set = [];
  }

  set.push(line);
}
program.push(set);

const program2 = program.map((instructions) => {
  const thing = {
    shift: 'left',
    value: 0,
  };
  for (const [i, instruction] of instructions.entries()) {
    const [command, a, b] = instruction.split(' ');
    if (command === 'div' && a === 'z') {
      if (b === '1') {
        const instruction2 = instructions[instructions.length - 3];
        if (instruction2) {
          const [_, __, b2] = instruction2.split(' ');
          thing.value = parseInt(b2);
        }
      } else if (b === '26') {
        const instruction2 = instructions[i + 1];
        if (instruction2) {
          thing.shift = 'right';
          const [_, __, b2] = instruction2.split(' ');
          thing.value = parseInt(b2);
        }
      }
    }
  }
  return thing;
});

console.log('program', program2);

//console.log('instructions', instructions);

type MemoryAddress = 'w' | 'x' | 'y' | 'z';
type Memory = Record<MemoryAddress, number>;

const zeroMemory: Memory = {
  w: 0,
  x: 0,
  y: 0,
  z: 16,
};

type MemoryNode = {
  memory: Memory;
  prev: MemoryNode | null;
};

let digits = [5, 1, 1, 4, 7, 1, 9, 1, 1, 6, 1, 2, 6, 1];
//            1  1  1 -1  1 -1 -1  1  1  1 -1 -1 -1 -1

let memory = {
  w: 0,
  x: 0,
  y: 0,
  z: 0,
};
let things = [];
for (const [i, digit] of digits.entries()) {
  memory = execute(memory, digit, program[i]);
  things.push(memory.z % 26);
  console.log(i + 1, digit, memory, (memory.z % 26) - digit);
}

console.log('string', digits.join(''));

function execute(memory: Memory, input: number, instructions: string[]) {
  const freshMemory = structuredClone(memory);
  for (const instruction of instructions) {
    const [command, a, b] = instruction.split(' ');

    const addr: MemoryAddress = a in freshMemory ? (a as MemoryAddress) : 'w';
    const p1 = freshMemory[addr];

    const p2 =
      b != null
        ? b in freshMemory
          ? freshMemory[b as MemoryAddress]
          : parseInt(b)
        : 0;

    switch (command) {
      case 'inp':
        freshMemory[addr] = input;
        break;
      case 'mul':
        freshMemory[addr] = p1 * p2;
        break;
      case 'add':
        freshMemory[addr] = p1 + p2;
        break;
      case 'mod':
        freshMemory[addr] = p1 % p2;
        break;
      case 'div':
        freshMemory[addr] = Math.floor(p1 / p2);
        break;
      case 'eql':
        freshMemory[addr] = p1 === p2 ? 1 : 0;
        break;
      default:
        throw new Error('invalid command');
    }
  }
  return freshMemory;
}

/*
 * z = w + 3
 * z = 12 + 26 * z + w
 * z = 9 + 26 * z + w
 * 73 47 21
 * last one
 * w = 9, z = 16
 */
