export class PriorityQueue<T> {
  private array: Array<T> = [];

  comparitor: (a: T) => number = Number;

  get size() {
    return this.array.length;
  }

  add(a: T) {
    const score = this.comparitor(a);
    const index = this.array.findIndex(
      b => score < this.comparitor(b),
    );
    if (index > -1) {
      this.array.splice(index, 0, a);
    } else {
      this.array.push(a);
    }
  }

  next(): T | void {
    return this.array.shift();
  }
}

