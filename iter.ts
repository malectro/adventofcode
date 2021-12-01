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
  for await (const item of iterable) {
    yield item;
    count++;
    if (count > limit) {
      return;
    }
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
  handler: (newChunk: C, item: C) => {chunk: C | null; buffer: C},
  initial: C,
) {
  let buffer = initial;
  for await (const item of iterable) {
    let step = handler(buffer, item);

    while (step.chunk) {
      yield step.chunk;
      buffer = step.buffer;
      step = handler(initial, buffer);
    }

    buffer = step.buffer;
  }
}

export function chunkLines(iter: AsyncIterableIterator<string>) {
  return reChunk(
    iter,
    (newChunk, item) => {
      const index = item.indexOf('\n');

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
    '',
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
