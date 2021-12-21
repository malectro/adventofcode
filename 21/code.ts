import {range} from '../iter.ts';

const player1Start = 8;
const player2Start = 3;

let player1 = {
  position: player1Start - 1,
  score: 0,
};

let player2 = {
  position: player2Start - 1,
  score: 0,
};

let players = [player1, player2];

let die = 0;
let rollCount = 0;

playGame();

console.log('done', players, die, rollCount);

const lower = players.reduce((loser, player) => player.score < loser.score ? player : loser);

console.log('answer', lower.score * rollCount);

function playGame() {
while (true) {
  for (const player of players) {
    for (const _ of range(0, 3)) {
      rollCount++;
      player.position += die + 1;
      die = (die + 1) % 100;
    }
    player.position = player.position % 10;
    player.score += player.position + 1;
    if (player.score >= 1000) {
      return;
    }
  }
}
}
