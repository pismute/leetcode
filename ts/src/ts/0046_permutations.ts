import assertEqual from './assert';

/*
 * [1,2,3]
 *
 *                 1                    2                           3          
 *           2          3         1          3                 1         2
 *           3          2         3          1                 2         3
 *
 */
function permute(nums: number[]): number[][] {
  const res: number[][] = [];

  function go(mask: number, cur: number[]) {
    if (mask === ((1 << nums.length) - 1)) {
      res.push(cur.slice());
    } else {
      for (let i = 0; i < nums.length; i++) {
        const pos: number = 1 << i;
        if ((mask & pos) === 0) {
          cur.push(nums[i]);
          go(mask | 1 << i, cur);
          cur.pop();
        }
      }
    }
  }

  go(0, []);
  return res;
};

assertEqual(permute([1, 2, 3]), [[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]]);
assertEqual(permute([0, 1]), [[0, 1], [1, 0]]);
assertEqual(permute([1]), [[1]]);



