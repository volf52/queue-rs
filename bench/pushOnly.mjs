import { Queue } from "../index.js";
import { JsQueue } from "./queue.mjs";

import { performance } from "perf_hooks";

const TOTAL = 100000;

const test = (q) => {
  const start = performance.now();

  for (let i = 0; i < TOTAL; i++) {
    q.push(i);
  }

  const end = performance.now();

  const diff = end - start;

  console.log(`${q.constructor.name} took ${diff} ms`);
};

test(new Queue());
test(new JsQueue());
