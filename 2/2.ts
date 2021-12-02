import { chunkLines, decodeText, map } from "../iter.ts";

const file = await Deno.open(new URL("input", import.meta.url));

let iter = Deno.iter(file);

let textIter = decodeText(iter);
textIter = chunkLines(textIter);

let commands = map(textIter, (string) => string.split(" "));

let aim = 0;
let distance = 0;
let depth = 0;
for await (const [command, valueString] of commands) {
  const value = parseInt(valueString);
  if (command === "forward") {
    distance += value;
    depth += value * aim;
  } else if (command === "down") {
    aim += value;
  } else if (command === "up") {
    aim -= value;
  }
}

console.log("result", distance * depth);

file.close();
