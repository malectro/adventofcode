import {decodeText, reChunk, map, reduce, range} from '../iter.ts';

const stdin = decodeText(Deno.iter(Deno.stdin));
const string = await reduce(stdin, (result, chunk) => result + chunk, '');

// literal
//const string = 'D2FE28';

// operator length type 0
//const string = '38006F45291200';

// operator length type 1
// const string = 'EE00D40C823060';

// example
//const string = 'A0016C880162017C3686B18A3D4780';

const binary = Array.from(string.trim())
  .map((char) => leftPad(parseInt(char, 16).toString(2), 4))
  .join('');

console.log('string', string);
console.log('binary', binary);

const result = parsePacket(binary);
console.log('result', result);

console.log('version count', countVersions(result));

type BasicPacket = {
  version: number;
  type: number;
  size: number;
};

interface LiteralPacket extends BasicPacket {
  type: 4;
  value: number;
}

interface OperatorPacket extends BasicPacket {
  subPackets: Packet[];
}

type Packet = OperatorPacket | LiteralPacket;

function parsePacket(data: string): Packet {
  let version = parseInt(data.slice(0, 3), 2);
  let type = parseInt(data.slice(3, 6), 2);

  //console.log(version, type, data.slice(3, 6));

  if (type === 4) {
    const [value, size] = parseLiteral(data.slice(6));
    return {
      version,
      type,
      value,
      size: size + 6,
    };
  }

  const lengthType = parseInt(data[6], 2);

  let index = 7;
  const subPackets = [];
  if (lengthType === 0) {
    const subPacketSize = parseInt(data.slice(index, index + 15), 2);
    index += 15;

    let i = 0;
    while (i < subPacketSize) {
      const subPacket = parsePacket(data.slice(i + index));
      subPackets.push(subPacket);
      i += subPacket.size;
    }

    index += i;
  } else {
    const subPacketCount = parseInt(data.slice(index, index + 11), 2);
    index += 11;

    for (const _ of range(0, subPacketCount)) {
      const subPacket = parsePacket(data.slice(index));
      subPackets.push(subPacket);
      index += subPacket.size;
    }
  }

  return {
    version,
    type,
    subPackets,
    size: index,
  };
}

function parseLiteral(data: string): [number, number] {
  let value = '';
  for (let i = 0; i < data.length; i += 5) {
    value += data.slice(i + 1, i + 5);
    if (data[i] === '0') {
      return [parseInt(value, 2), i + 5];
    }
  }
  throw new Error('Invalid literal');
}

function leftPad(string: string, length: number): string {
  if (length > string.length) {
    return (
      Array(length - string.length)
        .fill('0')
        .join('') + string
    );
  }
  return string;
}

function countVersions(packet: Packet): number {
  let sum = packet.version;

  if (packet.type === 4) {
    return sum;
  }

  return (packet as OperatorPacket).subPackets.reduce(
    (acc, packet) => countVersions(packet) + acc,
    sum,
  );
}
