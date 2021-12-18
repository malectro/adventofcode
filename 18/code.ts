import {decodeText, chunkLines, take, map, reduce, range} from '../iter.ts';

enum ReductionMode {
  Explode = 0,
  Split = 1,
}

const lines = chunkLines(decodeText(Deno.iter(Deno.stdin)));
const trees = map(lines, (line) => parsePairs(JSON.parse(line)));

const tree1 = (await trees.next()).value;

if (!tree1) {
  throw new Error('Must have at least one snail number.');
}

console.log(serializeTree(tree1));

let tree = tree1;
for await (const nextTree of trees) {
  tree = addTrees(tree, nextTree);
  tree = reduceTree(tree);
  console.log(serializeTree(tree));
}

console.log('magnitude', resolveMagnitude(tree));

type Pair = [PairElement, PairElement];
type PairElement = Pair | number;

interface PairNode {
  type: 'pair';
  left: Tree;
  right: Tree;
  parent: PairNode | null;
}
interface ValueNode {
  type: 'value';
  value: number;
  parent: PairNode | null;
}

type Tree = PairNode | ValueNode;

function parsePairs(pair: PairElement): Tree {
  if (typeof pair === 'number') {
    return {
      type: 'value',
      parent: null,
      value: pair,
    };
  }

  const left = parsePairs(pair[0]);
  const right = parsePairs(pair[1]);

  const self: PairNode = {
    type: 'pair',
    parent: null,
    left,
    right,
  };

  left.parent = self;
  right.parent = self;

  return self;
}

function addTrees(tree1: Tree, tree2: Tree): Tree {
  const newTree: PairNode = {
    type: 'pair',
    parent: null,
    left: tree1,
    right: tree2,
  };

  tree1.parent = newTree;
  tree2.parent = newTree;

  return newTree;
}

function reduceTree(tree: Tree): Tree {
  let modes: Array<ReductionMode> = [ReductionMode.Split, ReductionMode.Explode];

  let mode;
  while ((mode = modes.pop()) != null) {
    const queue = [{tree, depth: 0}];

    let thing;
    while ((thing = queue.pop()) != null) {
      //console.log('visiting', thing);
      const {tree, depth} = thing;
      if (tree.type === 'pair') {
        // @ts-ignore fix this?
        if (mode === ReductionMode.Explode && depth >= 4) {
          const {left, right} = tree;

          //console.log('depth 4', tree);

          if (left.type === 'value' && right.type === 'value') {
            //console.log('found exploder node', tree);

            // explode
            let nearestLeft = getNearestValue(tree, 'left');
            let nearestRight = getNearestValue(tree, 'right');

            if (nearestLeft) {
              nearestLeft.value += left.value;
            }
            if (nearestRight) {
              nearestRight.value += right.value;
            }

            const newNode = makeValueNode(0);
            spliceNode(tree, newNode);

            modes = [ReductionMode.Split, ReductionMode.Explode];
            break;
          }
        } else {
          queue.push(
            {tree: tree.right, depth: depth + 1},
            {
              tree: tree.left,
              depth: depth + 1,
            },
          );
        }
      } else if (
        mode === ReductionMode.Split &&
        tree.type === 'value' &&
        tree.value >= 10
      ) {
        const left = makeValueNode(Math.floor(tree.value / 2));
        const right = makeValueNode(Math.ceil(tree.value / 2));
        const newNode: PairNode = {
          type: 'pair',
          parent: null,
          left,
          right,
        };
        left.parent = newNode;
        right.parent = newNode;
        spliceNode(tree, newNode);
        modes = [ReductionMode.Split, ReductionMode.Explode];
        break;
      }
    }
  }

  return tree;
}

function getNearestValue(tree: Tree, dir: 'left' | 'right'): ValueNode | void {
  let prevNode = tree;
  let currentNode: Tree | null = tree.parent;

  while (currentNode != null) {
    if (currentNode.type === 'value') {
      return currentNode;
    }

    if (currentNode.parent === prevNode) {
      prevNode = currentNode;
      currentNode = currentNode[dir === 'left' ? 'right' : 'left'];
    } else if (currentNode[dir] !== prevNode) {
      prevNode = currentNode;
      currentNode = currentNode[dir];
    } else {
      prevNode = currentNode;
      currentNode = currentNode.parent;
    }
  }
}

function *getValueNodes(tree: Tree): IterableIterator<ValueNode> {

}

function spliceNode(oldNode: Tree, newNode: Tree) {
  const {parent} = oldNode;
  if (parent) {
    if (parent.left === oldNode) {
      parent.left = newNode;
    } else {
      parent.right = newNode;
    }
    newNode.parent = parent;
  }
}

function makeValueNode(value: number): ValueNode {
  return {
    type: 'value',
    parent: null,
    value,
  };
}

function resolveMagnitude(tree: Tree): number {
  if (tree.type === 'value') {
    return tree.value;
  }

  return 3 * resolveMagnitude(tree.left) + 2 * resolveMagnitude(tree.right);
}

/*
function *getLeafPairs(tree: Tree): IterableIterator<Tree> {
  if (tree.type === 'pair') {
    const {left, right} = tree;

    if (left.type === 'value' && right.type === 'value') {
      yield tree;
    } else if (left.type === 'pair') {
      yield *getLeafPairs(left);
    } else if (right.type === 'pair') {
      
    }
  }
}
*/

function serializeTree(tree: Tree): string {
  return JSON.stringify(treeToPairs(tree));
}

function treeToPairs(tree: Tree): PairElement {
  if (tree.type === 'value') {
    return tree.value;
  }

  return [treeToPairs(tree.left), treeToPairs(tree.right)];
}
