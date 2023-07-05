import assertEqual from './assert';

/*
 */
function fib(n: number): number {
  const dp = [0, 1];

  for (let i = 2; i <= n; i++) {
    dp[i % 2] = dp.reduce((a, b) => a + b);
  }

  return dp[n % 2] as number;
};

assertEqual(fib(2), 1);
assertEqual(fib(3), 2);
assertEqual(fib(4), 3);



