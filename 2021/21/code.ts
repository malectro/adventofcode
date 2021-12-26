import {range} from '../iter.ts';

/*
const player1Start = 4;
const player2Start = 8;
*/
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

let game = {
  die,
  rollCount,
  count: 1,
  players,
};

const game1 = playDeterministicGame(structuredClone(game));

const lower = game1.players.reduce((loser, player) =>
  player.score < loser.score ? player : loser,
);

console.log('part 1 answer', lower.score * game1.rollCount);

interface Game {
  die: number;
  rollCount: number;
  count: number;
  players: Array<{
    position: number;
    score: number;
  }>;
}

function playDeterministicGame(game: Game) {
  while (true) {
    for (const player of game.players) {
      for (const _ of range(0, 3)) {
        game.rollCount++;
        player.position += game.die + 1;
        game.die = (game.die + 1) % 100;
      }
      player.position = player.position % 10;
      player.score += player.position + 1;
      if (player.score >= 1000) {
        return game;
      }
    }
  }
  return game;
}

function getGameKey(game: Game): string {
  const [player1, player2] = game.players;
  return `${player1.position},${player1.score}:${player2.position},${player2.score}`;
}

let gameMap = new Map();
gameMap.set(getGameKey(game), game);

let winCounts = [0, 0];

const diceSums = new Map();
for (const d1 of range(1, 4)) {
  for (const d2 of range(1, 4)) {
    for (const d3 of range(1, 4)) {
      const sum = d1 + d2 + d3;
      diceSums.set(sum, (diceSums.get(sum) ?? 0) + 1);
    }
  }
}

console.log('sums', diceSums);

const maxSteps = 20;
let steps = 0;
while (gameMap.size > 0 && steps < maxSteps) {
  steps++;

  console.log('resolving games', gameMap.size);

  for (const playerId of range(0, 2)) {
    const newMap = new Map();
    for (const game of gameMap.values()) {
      for (const [roll, count] of diceSums) {
        const player = game.players[playerId];

        const position = (player.position + roll) % 10;
        const newPlayer = {
          position,
          score: player.score + position + 1,
        };

        const newGameCount = game.count * count;

        if (newPlayer.score >= 21) {
          winCounts[playerId] += newGameCount;
        } else {
          const newPlayers = game.players.slice();
          newPlayers[playerId] = newPlayer;

          const newGame = {
            ...game,
            players: newPlayers,
            count: newGameCount,
          };

          const key = getGameKey(newGame);
          const existingGame = newMap.get(key);

          if (existingGame) {
            existingGame.count += newGame.count;
          } else {
            newMap.set(key, newGame);
          }
        }
      }
    }
    gameMap = newMap;
  }
}

console.log('part 2 counts', winCounts);
