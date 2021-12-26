class PriorityQueue<T> {
  private array: Array<T> = [];

  comparitor: (a: T) => number = Number;

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

  toString() {
    return this.array.toString();
  }
}


const queue = new PriorityQueue();

queue.add(1);
queue.add(5);
queue.add(3);

console.log(queue, queue.next());
