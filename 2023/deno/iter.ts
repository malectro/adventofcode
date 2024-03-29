type Atom<T> = (iterator: AsyncIterableIterator<T>) => AsyncIterableIterator<T>;

/*
export function compose<T>(...atoms: Array<Atom<T>>) {
  return (iterator) => {
    for (const atom of atoms) {
      iterator = atom(iterator);
    }
    return iterator;
  };
}

export async function pipe<T>(
  iterator: AsyncIterableIterator<T>,
  ...atoms: Array<Atom<T>>
): Promise<void> {
  return compose(...atoms)(iterator);
}
*/

export async function* map<T, R>(
  iter: AsyncIterableIterator<T>,
  mapper: (thing: T) => R,
): AsyncIterableIterator<R> {
  for await (const item of iter) {
    yield mapper(item);
  }
}

/*
export function filter<T>(
  iter: AsyncIterableIterator<T | void>,
  test: Boolean,
): AsyncIterableIterator<T>;
*/
export async function* filter<T>(
  iter: AsyncIterableIterator<T>,
  test: (thing: T) => unknown,
): AsyncIterableIterator<T> {
  for await (const item of iter) {
    if (test(item)) {
      yield item;
    }
  }
}

export async function* mapNonNullable<T, R>(
  iter: AsyncIterableIterator<T>,
  mapper: (thing: T) => R,
): AsyncIterableIterator<NonNullable<R>> {
  for await (const item of iter) {
    let mapped = mapper(item);
    if (mapped != null) {
      // @ts-ignore type refinement was completed above
      yield mapped;
    }
  }
}

export function decodeText(
  iter: AsyncIterableIterator<Uint8Array>,
): AsyncIterableIterator<string> {
  const decoder = new TextDecoder();
  return map(iter, (data) => decoder.decode(data));
}

export async function* take<T>(
  iterable: AsyncIterableIterator<T>,
  limit: number,
): AsyncIterableIterator<T> {
  let count = 0;
  let next = await iterable.next();
  while (count < limit && !next.done) {
    yield next.value;
    count++;
    next = await iterable.next();
  }
}

export async function* takeWhile<T>(
  iterable: AsyncIterableIterator<T>,
  predicate: (thing: T) => unknown,
): AsyncIterableIterator<T> {
  let next = await iterable.next();
  while (!next.done && predicate(next.value)) {
    yield next.value;
    next = await iterable.next();
  }
}

export async function* buffer<T>(
  iterator: AsyncIterableIterator<T>,
  size: number,
): AsyncIterableIterator<Array<T>> {
  let buffer = [];

  for await (const item of iterator) {
    buffer.push(item);

    if (buffer.length === size) {
      yield buffer;
      buffer = [];
    }
  }

  yield buffer;
}

export async function toArray<T>(
  iter: AsyncIterableIterator<T>,
): Promise<Array<T>> {
  let array = [];
  for await (const item of iter) {
    array.push(item);
  }
  return array;
}

export async function* reChunk<C>(
  iterable: AsyncIterableIterator<C>,
  handler: (newChunk: C, item: C) => { chunk: C | null; buffer: C },
  initial: C,
) {
  let buffer = initial;
  for await (const item of iterable) {
    let step = handler(buffer, item);

    while (step.chunk != null) {
      yield step.chunk;
      buffer = step.buffer;
      step = handler(initial, buffer);
    }

    buffer = step.buffer;
  }

  if (buffer) {
    yield buffer;
  }
}

export function chunkLines(iter: AsyncIterableIterator<string>) {
  return reChunk(
    iter,
    (newChunk, item) => {
      const index = item.indexOf("\n");

      if (index >= 0) {
        return {
          chunk: newChunk + item.slice(0, index),
          buffer: item.slice(index + 1),
        };
      }

      return {
        chunk: null,
        buffer: newChunk + item,
      };
    },
    "",
  );
}

export async function* rollingWindow<T>(
  iterable: AsyncIterableIterator<T>,
  size: number,
): AsyncIterableIterator<Array<T>> {
  let window = [];

  // TODO (kyle): when there are fewer than size values
  for (let i = 0; i < size; i++) {
    window.push((await iterable.next()).value);
  }

  yield window;

  for await (const current of iterable) {
    window = [...window.slice(1), current];
    yield window;
  }
}

export async function reduce<T, R>(
  iterable: AsyncIterableIterator<T>,
  reducer: (acc: R, item: T) => R,
  initial: R,
): Promise<R> {
  let acc = initial;
  for await (const item of iterable) {
    acc = reducer(acc, item);
  }
  return acc;
}

export async function printIterator<T>(iter: AsyncIterableIterator<T>) {
  for await (const item of iter) {
    console.log(item);
  }
}

export function* range(from: number, to: number): IterableIterator<number> {
  const step = Math.sign(to - from);
  for (let i = from; i !== to; i += step) {
    yield i;
  }
}

export function clamp(number: number, floor: number, ceiling: number): number {
  return Math.max(Math.min(number, ceiling), floor);
}

export async function* enumerate<T>(
  iter: AsyncIterableIterator<T>,
): AsyncIterableIterator<[number, T]> {
  let i = 0;
  for await (const item of iter) {
    yield [i, item];
    i++;
  }
}
