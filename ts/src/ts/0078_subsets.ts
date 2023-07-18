import assertEqual from './assert';

/*
* [1,2,3]
*
*                                 []
*                           1                   []
*                    2          []          2           []
*                 3    []    3     []    3      []   3        []
*
*
*
* O(2^n), O(n)
*
* cache:
*
* O(n^2), O(n)
*/
function subsets(nums: number[]): number[][] {
  const cache = new Map<[number, number], number[][]>();

  function go(i: number, mask: number): number[][] {
    if (i == nums.length) {
      return [[]];
    }
    else if (cache.has([i, mask])) {
      return cache.get([i, mask]);
    } else {
      let res: number[][] = [];
      // include
      const res1 = go(i + 1, mask & (1 << i));
      for (const s of res1) {
        s.push(nums[i] as number);
      }
      res.push(...res1);

      res.push(...go(i + 1, mask)); // ignore

      cache.set([i, mask], res);
      return res;
    }
  }

  return go(0, 0);
};

assertEqual(subsets([1, 2, 3]), [[], [1], [2], [1, 2], [3], [1, 3], [2, 3], [1, 2, 3]]);
assertEqual(subsets([0]), [[], [0]]);



