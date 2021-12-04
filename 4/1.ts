const file = await Deno.readTextFile(new URL('input', import.meta.url));

const lines = file.split('\n');
const choices = lines[0].split(',');

const boardSize = 5;

let boardTime = 0;
let boardMarkers;
let winningBoard;
for (let i = 2; i < lines.length; i += 6) {
  const board = lines
    .slice(i, i + boardSize)
    .map((line) => line.split(' ').filter((n) => n));

  const result = playBoard(choices, board);

  if (result) {
    const {counts, choiceIndex, markers} = result;
    if (choiceIndex > boardTime) {
      boardTime = choiceIndex;
      boardMarkers = markers;
      winningBoard = board;
    }
  }
}

if (boardMarkers && winningBoard) {
  let score = 0;
  for (let i = 0; i < boardSize; i++) {
    for (let j = 0; j < boardSize; j++) {
      if (!boardMarkers[i][j]) {
        score += parseInt(winningBoard[i][j]);
      }
    }
  }
  score = score * parseInt(choices[boardTime]);
  console.log('score', score);
}

function playBoard(choices: string[], board: Array<Array<string>>) {
  const markers = board.map((row) => row.map((number) => false));
  const counts = range(boardSize * 2).map((_) => 0);

  for (const [choiceIndex, choice] of choices.entries()) {
    let found = false;

    for (let j = 0; j < boardSize; j++) {
      for (let k = 0; k < boardSize; k++) {
        if (board[j][k] === choice) {
          counts[j]++;
          counts[k + boardSize]++;

          markers[j][k] = true;
          found = true;
        }
      }
    }

    if (found) {
      for (let i = 0; i < counts.length; i++) {
        if (counts[i] === boardSize) {
          return {
            counts,
            choiceIndex,
            markers,
          };
        }
      }
    }
  }

  //console.log('counts', counts);
}

function range(size: number) {
  let array = [];
  for (let i = 0; i < size; i++) {
    array.push(i);
  }
  return array;
}
