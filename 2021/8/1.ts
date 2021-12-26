const source = 'input';
const file = await Deno.readTextFile(new URL(source, import.meta.url));

let displays = file
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

const uniqueDigits = new Set([1, 4, 7, 8]);

const uniqueDigitsByLength = new Map();
for (const digit of uniqueDigits) {
  const string = digits[digit];
  uniqueDigitsByLength.set(string.length, {digit: digit, string});
}

const digitsByLength = new Map();
for (const digit of digits) {
  let digits = digitsByLength.get(digit.length);
  if (!digits) {
    digits = [];
  }
  digits.push(digit);
  digitsByLength.set(digit.length, digits);
}

const uniqueDigitCounts = new Set(
  [...uniqueDigits].map((digit) => digits[digit].length),
);

const totalUniques = displays.reduce(
  (total, display) =>
    display.outputs.reduce(
      (total, output) => total + (uniqueDigitCounts.has(output.length) ? 1 : 0),
      total,
    ),
  0,
);

console.log('total uniques in output', totalUniques);

let finalNumber = 0;
for (const display of displays) {
  const {signals, outputs} = display;

  const allDigits = [...signals, ...outputs];

  // We're building a map of scrambled segments to their logical digits.
  const logicalDigits = new Map();

  // We know lists of segments of certain lengths are unique so we
  // add all of these to the logical mapping.
  for (const digit of allDigits) {
    const uniqueDigit = uniqueDigitsByLength.get(digit.length);
    if (uniqueDigit) {
      const scrambled = sortString(digit);
      logicalDigits.set(scrambled, {...uniqueDigit, scrambled});
    }
  }

  // We know for 6-segment numbers
  for (const digit of allDigits) {
    if (digit.length === 6) {
      const sortedString = sortString(digit);
      let result;
      if (
        // only 9 contains all the segments of 4, so any string containing it
        // maps to 9.
        segmentStringContains(
          sortedString,
          getLogicalDigit(logicalDigits, 4).scrambled,
        )
      ) {
        result = 9;
      } else if (
        // only 0 contains all the segments in 1
        segmentStringContains(
          sortedString,
          getLogicalDigit(logicalDigits, 1).scrambled,
        )
      ) {
        result = 0;
      } else {
        // and the remaining string must map to 6
        result = 6;
      }
      logicalDigits.set(sortedString, {
        digit: result,
        string: digits[result],
        scrambled: sortedString,
      });
    }
  }

  // Similarly for the 5-segment numbers
  for (const digit of allDigits) {
    if (digit.length === 5) {
      const sortedString = sortString(digit);
      let result;
      if (
        // only 3 contains all the segments of 1
        segmentStringContains(
          sortedString,
          getLogicalDigit(logicalDigits, 1).scrambled,
        )
      ) {
        result = 3;
      } else if (
        // only 5's segments are contained by 6
        segmentStringContains(
          getLogicalDigit(logicalDigits, 6).scrambled,
          sortedString,
        )
      ) {
        result = 5;
      } else {
        // and the remaining string must map to 2
        result = 2;
      }
      logicalDigits.set(sortedString, {
        digit: result,
        string: digits[result],
        scrambled: sortedString,
      });
    }
  }

  // now we calculate the output using the logical map.
  const stringOutput = outputs
    .map((string) => {
      const logicalDigit = logicalDigits.get(sortString(string));
      if (!logicalDigit) {
        throw new Error('uhoh');
      }
      return logicalDigit.digit;
    })
    .join('');
  const output = parseInt(stringOutput);

  //console.log('digitsByLength', digitsByLength);
  //console.log('logicalDigits', logicalDigits);
  //console.log(`${signals.join(' ')} | ${outputs.join(' ')}`);
  console.log('output', stringOutput, output);

  finalNumber += output;
}

console.log('final number', finalNumber);

function sortString(string: string): string {
  return string.split('').sort().join('');
}

function segmentStringContains(string1: string, string2: string): boolean {
  const set = new Set(string1);
  return [...string2].every((letter) => set.has(letter));
}

type LogicalDigit = {digit: number; string: string; scrambled: string};
function getLogicalDigit(
  logicalDigits: Map<string, LogicalDigit>,
  digit: number,
): LogicalDigit {
  return [...logicalDigits.values()].filter((item) => item.digit === digit)[0];
}
