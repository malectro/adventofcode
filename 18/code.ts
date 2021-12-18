import {
  decodeText,
  chunkLines,
  take,
  map,
  reduce,
  range,
  toArray,
} from '../iter.ts';

enum ReductionMode {
  Explode = 0,
  Split = 1,
}

const lines = chunkLines(decodeText(Deno.iter(Deno.stdin)));
const trees = await toArray(map(lines, (line) => parsePairs(JSON.parse(line))));

const tree1 = trees[0];

if (!tree1) {
  throw new Error('Must have at least one snail number.');
}

console.log(serializeTree(tree1));

let tree = tree1;
for (const nextTree of trees.slice(1)) {
  tree = addTrees(tree, nextTree);
  tree = reduceTree(tree);
  console.log(serializeTree(tree));
}

console.log('magnitude', resolveMagnitude(tree));

const largest = trees.reduce((acc, tree1) => {
  return trees.reduce((acc, tree2) => {
    return Math.max(
      tree1 !== tree2
        ? resolveMagnitude(reduceTree(addTrees(tree1, tree2)))
        : 0,
      acc,
    );
  }, acc);
}, 0);

console.log('largest', largest);

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
    left: structuredClone(tree1),
    right: structuredClone(tree2),
  };

  newTree.left.parent = newTree;
  newTree.right.parent = newTree;

  return newTree;
}

function reduceTree(tree: Tree): Tree {
  let modes: Array<ReductionMode> = [
    ReductionMode.Split,
    ReductionMode.Explode,
  ];

  tree = structuredClone(tree);

  let mode;
  while ((mode = modes.pop()) != null) {
    const queue = [{tree, depth: 0}];

    let thing;
    while ((thing = queue.pop()) != null) {
      const {tree, depth} = thing;

      if (tree.type === 'pair') {
        if (mode === ReductionMode.Explode && depth >= 4) {
          const {left, right} = tree;

          if (left.type === 'value' && right.type === 'value') {
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

function* getValueNodes(tree: Tree): IterableIterator<ValueNode> {}

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

function serializeTree(tree: Tree): string {
  return JSON.stringify(treeToPairs(tree));
}

function treeToPairs(tree: Tree): PairElement {
  if (tree.type === 'value') {
    return tree.value;
  }

  return [treeToPairs(tree.left), treeToPairs(tree.right)];
}
