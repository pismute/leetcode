import assert from 'node:assert';

/*
 * [1,2,3]
 *
 *                 1                    2                           3          
 *           2          3         1          3                 1         2
 *           3          2         3          1                 2         3
 *
 */
function permute(nums: number[]): number[][] {
  return [[1]];
};


assert(permute([1, 2, 3]) === [[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]]);
assert(permute([0, 1]) === [[0, 1], [1, 0]]);
assert(permute([1]) === [[1]]);



