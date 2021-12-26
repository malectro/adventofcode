9, 0
function part1(w: number, z: number = 0) {
  let x = z;
  z = Math.floor(z / 1);
  if (w !== x % 26 + 13) {
    z *= 26;
    z += w + 3;
  }
  return z
}

9, 12
function part2(w: number, z: number) {
  let x = z;
  z = Math.floor(z / 1);
  if (w !== x % 26 + 11) {
    z *= 26;
    z += w + 12;
  }
  return z
}

function part3(w: number, z: number) {
  let x = z;
  z = Math.floor(z / 1);
  if (w !== x % 26 + 15) {
    z *= 26;
    z += w + 9;
  }
  return z
}

function part4(w: number, z: number) {
  let x = z;
  z = Math.floor(z / 26);
  if (w !== x % 26 - 6) {
    z *= 26;
    z += w + 12;
  }
  return z
}

function part12(w: number, z: number) {
  let x = z;
  z = Math.floor(z / 26);
  if (w !== x % 26 - 0) {
    z *= 26;
    z += w + 11;
  }
  return z
}

function part13(w: number, z: number) {
  let x = z;
  z = Math.floor(z / 26);
  if (w !== x % 26 - 8) {
    z *= 26;
    z += w + 10;
  }
  return z
}

function part14(w: number, z: number) {
  let result = Math.floor(z / 26);
  console.log('what', result);
  if (w !== z % 26 - 7) {
    result *= 26;
    result += w + 3;
  }
  return result;
}

console.log('hi', part14(9, 16));
