import test from "ava";

import { sum, Queue } from "../index.js";

test.beforeEach("generate queue object", (t) => {
  t.context.q = new Queue();
});

test("sum from native", (t) => {
  t.is(sum(1, 2), 3);
});

test("queue is constructed with 0 len", (t) => {
  /**
   * @type {Queue}
   */
  const q = t.context.q;

  t.is(q.len(), 0);
  t.true(q.isEmpty());
});

test("queue length should increase by one on push back", (t) => {
  /**
   * @type {Queue}
   */
  const q = t.context.q;

  t.true(q.isEmpty());

  q.push(2);

  t.is(q.len(), 1);
});

test("pop on empty returns none", (t) => {
  /**
   * @type {Queue}
   */
  const q = t.context.q;

  t.is(q.pop(), null);
});

test("push and pop in succession return the same val", (t) => {
  /**
   * @type {Queue}
   */
  const q = t.context.q;
  const val = 2;

  q.push(val);

  t.is(q.pop(), val);
});

test("queue is fifo", (t) => {
  /**
   * @type {Queue}
   */
  const q = t.context.q;
  const val1 = 13;
  const val2 = 42;

  q.push(val1);
  q.push(val2);

  t.is(q.pop(), val1);
  t.is(q.pop(), val2);
});
