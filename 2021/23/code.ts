import {PriorityQueue} from '../utils.ts';

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

type Amphi = 'A' | 'B' | 'C' | 'D';

const costs = {
  A: 1,
  B: 10,
  C: 100,
  D: 1000,
};

const directions = [
  {x: 1, y: 0},
  {x: -1, y: 0},
  {x: 0, y: 1},
  {x: 0, y: -1},
];

const hallLocations = {
  A: 2,
  B: 4,
  C: 6,
  D: 8,
};

const hallLocationsSet = new Set(Object.values(hallLocations));

interface Point {
  x: number;
  y: number;
}
interface RoomTile {
  type: 'room';
  position: Point;
  name: Amphi;
  value: Amphi | null;
}
interface HallTile {
  type: 'hall';
  position: Point;
  value: Amphi | null;
}
interface WallTile {
  type: 'wall';
  position: Point;
}

type Tile = WallTile | HallTile | RoomTile;

type Game = Tile[][];

const game: Game = input.split('\n').map((line, y) => {
  const rooms: Amphi[] = ['A', 'B', 'C', 'D'];
  let roomIndex = 0;
  return Array.from(line).map((char, x) => {
    const position = {x, y};

    if (/[ABCD]/.test(char)) {
      return {
        position,
        type: 'room',
        value: char as Amphi,
        name: rooms[roomIndex++],
      };
    }

    return char === '#'
      ? {
          position,
          type: 'wall',
        }
      : char === ' '
      ? {
          position,
          type: 'wall',
        }
      : {
          position,
          type: 'hall',
          value: null,
        };
  });
});

const gamesToPlay = new PriorityQueue<{
  energy: number;
  game: Game;
}>();
gamesToPlay.comparitor = ({energy}) => energy;

const gameScores = new Map();

/*
const tile1 = game[2][3];
const tile2 = game[1][1];
[tile1.value, tile2.value] = [tile2.value, tile1.value];

const tile3 = game[2][5];
const tile4 = game[1][6];
[tile3.value, tile4.value] = [tile4.value, tile3.value];
*/

//console.log('playable tiles', game[1][1], getMoveableTiles(game, game[1][1]));
console.log(serializeGame(game));
const finalScore = playGame(game);

console.log('final score', finalScore);

function playGame(game: Game): number {
  let energy = 0;

  gamesToPlay.add({game, energy});

  let steps = 0;
  let item;
  while ((item = gamesToPlay.next()) && steps < 1) {
    //steps++;

    const {game, energy} = item;
    //console.log('resolving', serializeGame(game));

    if (isGameOver(game)) {
      return energy;
    }

    let playableTiles = [];
    for (const row of game) {
      for (const tile of row) {
        if ('value' in tile && tile.value) {
          if (!(tile.type === 'room' && isTileDone(game, tile))) {
            playableTiles.push(tile);
          }
        }
      }
    }

    for (const tile of playableTiles) {
      // @ts-ignore a cost will always exist here
      const cost = costs[tile.value];

      const isInRoom = tile.type === 'room';

      let possibleMoves = getMoveableTiles(game, tile);
      if (isInRoom) {
        possibleMoves = possibleMoves.filter(
          ({tile}) =>
            tile.type === 'hall' && !hallLocationsSet.has(tile.position.x),
        );
      } else {
        //console.log('looking at hall tile', tile.value);
        const roomTiles = getRoomTiles(game, tile.value);
        if (
          roomTiles.some(
            (roomTile) =>
              roomTile.value != null && roomTile.value !== roomTile.name,
          )
        ) {
          possibleMoves = [];
        } else {
          possibleMoves = possibleMoves.filter(
            (move) =>
              move.tile.type === 'room' && move.tile.name === tile.value,
          );
          if (possibleMoves.length > 1) {
            possibleMoves = possibleMoves.slice(-1);
          }
        }
        //console.log('possiblemoves', possibleMoves);
      }

      for (const {distance, tile: moveTile} of possibleMoves) {
        const score = distance * cost + energy;

        [tile.value, moveTile.value] = [moveTile.value, tile.value];
        const key = serializeGame(game);

        const prevCost = gameScores.get(key);
        if (prevCost == null || score < prevCost) {
          gameScores.set(key, score);
          gamesToPlay.add({
            energy: score,
            game: structuredClone(game),
          });
        }

        [tile.value, moveTile.value] = [moveTile.value, tile.value];
      }
    }
  }

  //console.log('gamesToPlay', gamesToPlay.size);
  console.log('scores', gameScores);
  //console.log('hi', gamesToPlay.array.map(item => item.energy));

  /*
  console.log('next game', gamesToPlay.next().energy);
  console.log('next game', gamesToPlay.next().energy);
  console.log('next game', gamesToPlay.next().energy);
  console.log('next game', gamesToPlay.next().energy);
  console.log('next game', gamesToPlay.next().energy);
  */

  throw new Error('failed to find a solution');
}

function isGameOver(grid: Game): boolean {
  for (const row of grid) {
    for (const tile of row) {
      if (tile.type === 'room' && tile.name !== tile.value) {
        return false;
      }
    }
  }

  return true;
}

//function getMoveableTiles(grid: Game, tile:

function serializeGame(game: Game): string {
  return (
    game[1]
      .slice(1, -1)
      .filter((tile): tile is HallTile => tile.type === 'hall')
      .map((tile) => tile.value || '.')
      .join('') +
    game[2]
      .filter((tile): tile is RoomTile => tile.type === 'room')
      .map((tile) => tile.value || '.')
      .join('') +
    game[3]
      .filter((tile): tile is RoomTile => tile.type === 'room')
      .map((tile) => tile.value || '.')
      .join('')
  );
}

function getMoveableTiles(
  game: Game,
  startTile: RoomTile | HallTile,
): Array<{distance: number; tile: HallTile | RoomTile}> {
  let distance = 0;
  let tilesVisited = new Set<HallTile | RoomTile>();
  let tilesToCheck = [startTile];

  let tile;
  while ((tile = tilesToCheck.shift())) {
    for (const direction of directions) {
      const nextTile =
        game[tile.position.y + direction.y]?.[tile.position.x + direction.x];
      if (
        nextTile &&
        nextTile !== startTile &&
        nextTile.type !== 'wall' &&
        nextTile.value === null &&
        !tilesVisited.has(nextTile)
      ) {
        tilesToCheck.push(nextTile);
        tilesVisited.add(nextTile);
      }
    }
  }

  return Array.from(tilesVisited).map((tile) => ({
    tile,
    distance: getDistance(startTile.position, tile.position),
  }));
}

function getRoomTiles(game: Game, name: Amphi): RoomTile[] {
  let result = [];
  for (const row of game) {
    for (const tile of row) {
      if (tile.type === 'room' && tile.name === name) {
        result.push(tile);
      }
    }
  }
  return result;
}

function getDistance(point1: Point, point2: Point): number {
  return Math.abs(point2.x - point1.x) + Math.abs(point2.y - point1.y);
}

function isRoomComplete(game: Game, tile: RoomTile): boolean {
  let otherTile =
    tile.position.y === 3 ? game[2][tile.position.x] : game[3][tile.position.x];
  return tile.name === tile.value && otherTile.name === tile.value;
}

function isTileDone(game: Game, tile: RoomTile): boolean {
  let tileDone = tile.name === tile.value;

  if (tile.position.y === 2) {
    const otherTile = game[3][tile.position.x];
    tileDone = tileDone && otherTile.name === otherTile.value;
  }

  return tileDone;
}

/*

try 1
#############
#...........#
###A#C#B#B###
  #D#D#A#C#
  #########

3
#############
#A..........#
###.#C#B#B###
  #D#D#A#C#
  #########

3 + 30 = 33
#############
#A.........B#
###.#C#B#.###
  #D#D#A#C#
  #########

40 + 33 = 73
#############
#A........BB#
###.#C#.#.###
  #D#D#A#C#
  #########

73 + 7 = 80
#############
#AA.......BB#
###.#C#.#.###
  #D#D#.#C#
  #########


80 + 1000 = 1080
#############
#AA.......BB#
###.#.#C#.###
  #D#D#C#.#
  #########

1080 + 17000 = 18080
#############
#AA.......BB#
###.#.#C#D###
  #.#.#C#D#
  #########

18080 + 140 = 18220
#############
#AA.........#
###.#B#C#D###
  #.#B#C#D#
  #########

18226
#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########


try 2
#############
#...........#
###A#C#B#B###
  #D#D#A#C#
  #########

9
#############
#..........A#
###.#C#B#B###
  #D#D#A#C#
  #########

9 + 80 = 89
#############
#.B........A#
###.#C#B#.###
  #D#D#A#C#
  #########

89 + 40 = 129
#############
#..BB......A#
###.#C#.#.###
  #D#D#A#C#
  #########

129 + 5 = 134
#############
#..BB.....AA#
###.#C#.#.###
  #D#D#.#C#
  #########

134 + 1000 = 1134
#############
#..BB.....AA#
###.#.#C#.###
  #D#D#C#.#
  #########

1134 + 8000 = 9134
#############
#.BB......AA#
###.#.#C#.###
  #D#.#C#D#
  #########

9134 + 80 + 9000 = 18214 + 18 = 18232
#############
#.........AA#
###.#B#C#D###
  #.#B#C#D#
  #########

18158
#############
#BB.......AA#
###.#.#C#D###
  #.#.#C#D#
  #########

18158 + 100
18258
#############
#.........AA#
###.#B#C#D###
  #.#B#C#D#
  #########

19226
#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########

D
10000 + 7000
C
500 + 500

18000



*/
