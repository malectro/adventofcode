export function* take<T>(
  iterable: IterableIterator<T>,
  limit: number,
): IterableIterator<T> {
  let count = 0;
  let next = iterable.next();
  while (count < limit && !next.done) {
    yield next.value;
    count++;
    next = iterable.next();
  }
}

export function* rollingWindow<T>(
  iterable: IterableIterator<T>,
  size: number,
): IterableIterator<Array<T>> {
  let window = [];

  // TODO (kyle): when there are fewer than size values
  for (let i = 0; i < size; i++) {
    const next = iterable.next();
    if (next.done) {
      return;
    }
    window.push(next.value);
  }

  yield window;

  for (const current of iterable) {
    window = [...window.slice(1), current];
    yield window;
  }
}

/*
export function* slidingWindow<T>(
  iterable: IterableIterator<T>,
  size: number,
): IterableIterator<Array<T | void>> {

}
*/
