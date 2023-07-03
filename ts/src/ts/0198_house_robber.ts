import assertEqual from './assert';

/*
 *
 * [1, 2, 3, 1]
 *
 * (1,0), (1, 2), (4, 2), (4, 3)
 *
 * O(n), O(2)
 *
 * [2, 1, 1, 2]
 * (2, 0), (2, 1), (3, 2), (3, 4)
 */
function rob(nums: number[]): number {
  const dp = [0, 0];

  for (let i = 0; i < nums.length; i++) {
    const max = Math.max.apply(null, dp);
    dp[i % 2] += nums[i];
    dp[(i + 1) % 2] = max;
  }

  return Math.max.apply(null, dp);
};

assertEqual(rob([1, 2, 3, 1]), 4)
assertEqual(rob([2, 7, 9, 3, 1]), 12)



