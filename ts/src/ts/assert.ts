import assert from 'node:assert';
import { deepEqual } from './deep-equal';

export default function assertEqual(a: any, b: any, msg: string = "assert") {
  const start = performance.now();

  if (Array.isArray(a) && Array.isArray(b)) {
    const aa = a.slice();
    const bb = b.slice();
    aa.sort()
    bb.sort()
    assert(deepEqual(aa, bb), `${a} == ${b}`);
  } else {
    assert.equal(a, b, `${a} == ${b}`);
  }
  const end = performance.now();
  console.log(`${msg}: Execution time: ${end - start} ms`);
}
