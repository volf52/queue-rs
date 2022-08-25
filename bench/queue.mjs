export class JsQueue {
  q = [];

  push(v) {
    this.q.push(v);
  }

  pop() {
    return this.q.shift();
  }
}
