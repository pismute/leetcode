import deepEqual from 'deep-equal';
import assert from 'node:assert';

function sort(arr: number[][]): number[][] {
  arr.sort();
  return arr;
}

export default function assertEqual(a: any, b: any, msg: string = "assert") {
  const start = performance.now();
  if (Array.isArray(a)) {
    assert(deepEqual(sort(a), sort(b), { strict: true }), `${a} == ${b}`);
  } else {
    assert.equal(a, b);
  }
  const end = performance.now();
  console.log(`${msg}: Execution time: ${end - start} ms`);
}
