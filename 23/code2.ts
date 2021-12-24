import {PriorityQueue} from '../utils.ts';
import {range} from '../iter.ts';

/*
const input = `
#############
#...........#
###B#A#C#D###
  #A#B#C#D#
  #########`.trim();
  */
const input = `
#############
#...........#
###A#C#B#B###
  #D#D#A#C#
  #########`.trim();
/*
const input = `
#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########`.trim();
  */
const lines = input.split('\n');

type Amphi = 'A' | 'B' | 'C' | 'D';

const costs = {
  A: 1,
  B: 10,
  C: 100,
  D: 1000,
};

const hallLocations = {
  A: 2,
  B: 4,
  C: 6,
  D: 8,
};

const hallLocationsSet = new Set(Object.values(hallLocations));

const game = {
  hall: Array(lines[0].length - 2).fill(null),
  rooms: {
    A: ['A', 'D', 'D', 'D'],
    B: ['C', 'C', 'B', 'D'],
    C: ['B', 'B', 'A', 'A'],
    D: ['B', 'A', 'C', 'C'],
  },
};
/*
const game = {
  hall: Array(lines[0].length - 2).fill(null),
  rooms: {
    A: ['A', 'D'],
    B: ['C', 'D'],
    C: ['B', 'A'],
    D: ['B', 'C'],
  },
};
*/
/*
const game = {
  hall: Array(lines[0].length - 2).fill(null),
  rooms: {
    A: ['B', 'A'],
    B: ['A', 'B'],
    C: ['C', 'C'],
    D: ['D', 'D'],
  },
};
*/

console.log('final', playGame(game));

function playGame(game) {
  const queue = new PriorityQueue();
  queue.comparitor = ({energy}) => energy;

  queue.add({game, energy: 0});

  const energyMap = new Map();
  energyMap.set(JSON.stringify(game), 0);

  function makeMove(newGame, newEnergy) {
    const key = JSON.stringify(newGame);
    const prevEnergy = energyMap.get(key) ?? Infinity;

    if (newEnergy < prevEnergy) {
      queue.add({game: newGame, energy: newEnergy});
      energyMap.set(key, newEnergy);
    }
  }

  let round;
  while ((round = queue.next())) {
    const {game, energy} = round;

    if (isGameOver(game)) {
      console.log('we win', serializeGame(game));
      return energy;
    }

    // can a room be moved into, or, should a piece be moved out of the room?
    const roomStatus = Object.fromEntries(
      Object.entries(game.rooms).map(([name, room]) => [
        name,
        room.every((value) => value === null || value === name),
      ]),
    );

    const newGames = [];

    for (const [x, value] of game.hall.entries()) {
      if (value) {
        if (roomStatus[value]) {
          const room = game.rooms[value];

          const hallLocation = hallLocations[value];
          const diff = hallLocation - x;
          const sign = Math.sign(diff);

          let isPathClear = true;
          for (const i of range(x + sign, hallLocation + sign)) {
            if (game.hall[i] !== null) {
              isPathClear = false;
            }
          }

          if (isPathClear) {
            const newGame = structuredClone(game);
            const index = room.findIndex((value) => value === null);

            newGame.rooms[value][index] = value;
            newGame.hall[x] = null;

            const distance = Math.abs(diff) + index + 1;
            const newEnergy = costs[value] * distance + energy;

            makeMove(newGame, newEnergy);
          }
        }
      }
    }

    for (const [name, room] of Object.entries(game.rooms)) {
      if (!roomStatus[name]) {
        const firstIndex = room.findIndex((value) => value !== null);
        const value = room[firstIndex];

        const hallLocation = hallLocations[name];

        const possibleMoves = [];
        for (const x of range(hallLocation + 1, game.hall.length)) {
          if (game.hall[x] === null) {
            possibleMoves.push(x);
          } else {
            break;
          }
        }
        for (const x of range(hallLocation - 1, -1)) {
          if (game.hall[x] === null) {
            possibleMoves.push(x);
          } else {
            break;
          }
        }

        for (const move of possibleMoves) {
          if (hallLocationsSet.has(move)) {
            continue;
          }

          const newGame = structuredClone(game);

          newGame.hall[move] = value;
          newGame.rooms[name][firstIndex] = null;

          const distance = Math.abs(move - hallLocation) + firstIndex + 1;
          const newEnergy = costs[value] * distance + energy;

          makeMove(newGame, newEnergy);
        }
      }
    }
  }

  throw new Error('failed to win a game');
}

function isGameOver(game) {
  for (const [name, room] of Object.entries(game.rooms)) {
    for (const value of room) {
      if (name !== value) {
        return false;
      }
    }
  }
  return true;
}

function serializeGame(game) {
  return (
    game.hall.map((value) => (value == null ? '.' : value)).join('') +
    Object.values(game.rooms)
      .map((room) =>
        room.map((value) => (value == null ? '.' : value)).join(''),
      )
      .join('')
  );
}
