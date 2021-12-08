const source = 'input';
const file = await Deno.readTextFile(new URL(source, import.meta.url));

const displays = file
  .trim()
  .split('\n')
  .map((text) => text.split('|'))
  .map(([signals, outputs]) => ({
    signals: signals.trim().split(' '),
    outputs: outputs.trim().split(' '),
  }));

const digits = [
  'abcefg',
  'cf',
  'acdeg',
  'acdfg',
  'bcdf',
  'abdfg',
  'abdefg',
  'acf',
  'abcdefg',
  'abcdfg',
];

const uniqueDigits = [1, 4, 7, 8];

const uniqueDigitCounts = new Set(
  uniqueDigits.map((digit) => digits[digit].length),
);

console.log('uniqueDigitCounts', uniqueDigitCounts);

const totalUniques = displays.reduce(
  (total, display) =>
    display.outputs.reduce(
      (total, output) => total + (uniqueDigitCounts.has(output.length) ? 1 : 0),
      total,
    ),
  0,
);

console.log('total uniques', totalUniques);
