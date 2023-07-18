import assert from 'node:assert';
import { deepEqual } from './deep-equal';

function sorted<T>(array: T[]): T[] {
  const arr = array.slice();

  if (Object.hasOwn(arr, 0) && Array.isArray(arr[0])) {
    for (let i = 0; i < arr.length; i++) {
      arr[i] = arr[i].slice();
      arr[i].sort();
    }
  }

  arr.sort();
  return arr;
}

export default function assertEqual(a: any, b: any, msg: string = "assert") {
  const start = performance.now();

  if (Array.isArray(a) && Array.isArray(b)) {
    const aa = sorted(a);
    const bb = sorted(b);

    assert(deepEqual(aa, bb), `${a} == ${b}`);
  } else {
    assert.equal(a, b, `${a} == ${b}`);
  }
  const end = performance.now();
  console.log(`${msg}: Execution time: ${end - start} ms`);
}
