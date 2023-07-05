import assert from 'node:assert';
import { deepEqual } from './deep-equal';

function sort(arr: number[][]): number[][] {
  arr.sort();
  return arr;
}

export default function assertEqual(a: any, b: any, msg: string = "assert") {
  const start = performance.now();
  if (Array.isArray(a)) {
    assert(deepEqual(sort(a), sort(b)), `${a} == ${b}`);
  } else {
    assert.equal(a, b, `${a} == ${b}`);
  }
  const end = performance.now();
  console.log(`${msg}: Execution time: ${end - start} ms`);
}
