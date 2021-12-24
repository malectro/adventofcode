import {slidingWindows} from 'https://deno.land/std@0.119.0/collections/sliding_windows.ts';
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

const digits = [];

const program = [];
let set: string[] = [];
for (const [i, line] of lines.entries()) {
  if (line === 'inp w' && set.length > 0) {
    program.push(set);
    set = [];
  }

  set.push(line);
}

//console.log('instructions', instructions);

type MemoryAddress = 'w' | 'x' | 'y' | 'z';

for (const instructions of program.slice(0, 1)) {
  let max = 1;
  for (const digit of range(1, 10)) {
    const memory: Record<MemoryAddress, number> = {
      w: 0,
      x: 0,
      y: 0,
      z: 0,
    };
    console.log('trying digit', digit);
    for (const instruction of instructions) {
      const [command, a, b] = instruction.split(' ');

      const addr: MemoryAddress = a in memory ? a as MemoryAddress : 'w';
      const p1 = memory[addr];

      const p2 = b != null ? (b in memory ? memory[b as MemoryAddress] : parseInt(b)) : 0;

      switch (command) {
        case 'inp':
          memory[addr] = digit;
          break;
        case 'mul':
          memory[addr] = p1 * p2;
          break;
        case 'add':
          memory[addr] = p1 + p2;
          break;
        case 'mod':
          memory[addr] = p1 % p2;
          break;
        case 'div':
          memory[addr] = Math.floor(p1 / p2);
          break;
        case 'eql':
          memory[addr] = p1 === p2 ? 1 : 0;
          break;
        default:
          throw new Error('invalid command');
      }
      console.log(instruction, memory);
    }
    if (memory.z === 0) {
      max = Math.max(digit, max);
    }
  }
  digits.push(max);
}

console.log('digits', digits);
