const file = await Deno.readTextFile(new URL('input', import.meta.url));

const fish = file
  .trim()
  .split(',')
  .map((text) => parseInt(text));

for (let i = 0; i < 80; i++) {
  let length = fish.length;
  for (let j = 0; j < length; j++) {
    if (fish[j] === 0) {
      fish[j] = 6;
      fish.push(8);
    } else {
      fish[j]--;
    }
  }
}

console.log('fish count', fish.length);
