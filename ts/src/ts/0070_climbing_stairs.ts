import assertEqual from './assert';

/*
 * n = 3
 *
 * [0, 1, 2, 3]
 *  2  2  2  2 = 2^n
 *
 * brute force
 * O(2^n), O(n)
 *
 * dp: top down
 *
 * [0, 1, 2, 3]
 *  1  1  2  3
 *
 * O(n), O(2)
 */
function climbStairs(n: number): number {
  const dp: number[] = new Array(2).fill(0);

  dp[0] = 1;

  for (let i = 1; i <= n; i++) {
    dp[i % 2] += dp[(i + 1) % 2] as number;
  }

  return dp[n % 2] as number;
};

assertEqual(climbStairs(2), 2);
assertEqual(climbStairs(3), 3);



