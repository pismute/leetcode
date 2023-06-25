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
  return [[1]];
};

assertEqual(permute([1, 2, 3]), [[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]]);
assertEqual(permute([0, 1]), [[0, 1], [1, 0]]);
assertEqual(permute([1]), [[1]]);



