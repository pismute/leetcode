import assertEqual from './assert';

/*
 *
 * [1, 100, 1, 1
 *  (1, 0), (1, 100), (2, 100), (2, 3)
 *
 * O(n), O(2)
 */
function minCostClimbingStairs(cost: number[]): number {
  const dp = Array(2).fill(0);

  for (let i = 0; i < cost.length; i++) {
    dp[i % 2] = Math.min.apply(null, dp) + cost[i];
  }

  return Math.min.apply(null, dp);
};

assertEqual(minCostClimbingStairs([10, 15, 20]), 15);
assertEqual(minCostClimbingStairs([1, 100, 1, 1, 1, 100, 1, 1, 100, 1]), 6);




