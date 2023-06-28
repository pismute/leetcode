import assertEqual from './assert';

/*
 */
function tribonacci(n: number): number {
  const dp = [0, 1, 1];

  for (let i = 3; i <= n; i++) {
    dp[i % 3] = dp.reduce((x, y) => x + y);
  }

  return dp[n % 3];
};

assertEqual(tribonacci(4), 4);
assertEqual(tribonacci(25), 1389537);



