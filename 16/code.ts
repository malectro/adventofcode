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
//const string = '9C0141080250320F1802104A08';

const binary = Array.from(string.trim())
  .map((char) => leftPad(parseInt(char, 16).toString(2), 4))
  .join('');

console.log('string', string);
console.log('binary', binary);

const packet = parsePacket(binary);
console.log('packet', packet);

console.log('version count', countVersions(packet));

console.log('result', resolvePacket(packet));

type BasicPacket = {
  version: number;
  size: number;
};

interface LiteralPacket extends BasicPacket {
  type: 4;
  value: number;
}

interface OperatorPacket extends BasicPacket {
  type: 0 | 1 | 2 | 3 | 5 | 6 | 7;
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
    // @ts-ignore fix this
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

function resolvePacket(packet: Packet): number {
  if (packet.type === 4) {
    return packet.value;
  }

  const subValues = packet.subPackets.map(resolvePacket);

  switch (packet.type) {
    case 0:
      return subValues.reduce((sum, value) => sum + value, 0);
    case 1:
      return subValues.reduce((sum, value) => sum * value, 1);
    case 2:
      return Math.min(...subValues);
    case 3:
      return Math.max(...subValues);
    case 5:
      return subValues[0] > subValues[1] ? 1 : 0;
    case 6:
      return subValues[0] < subValues[1] ? 1 : 0;
    case 7:
      return subValues[0] === subValues[1] ? 1 : 0;
    default:
      throw new Error('Invalid packet type');
  }
}
