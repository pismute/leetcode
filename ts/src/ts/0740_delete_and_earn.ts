import assertEqual from './assert';

function deleteAndEarn(nums: number[]): number {
  const counts = nums.reduce<Map<number, number>>((acc, x) => {
    const v = acc.get(x);
    acc.set(x, v === undefined ? 1 : v + 1);
    return acc;
  }, new Map<number, number>())

  let min = Number.MAX_SAFE_INTEGER;
  let max = Number.MIN_SAFE_INTEGER;

  for (let k of counts.keys()) {
    min = Math.min(min, k);
    max = Math.max(max, k);
  }
  const dp = [0, 0];

  for (let i = min; i <= max; i++) {
    const m = Math.max(...dp);
    if (counts.has(i)) {
      const v = counts.get(i) as number;
      dp[i % 2] = dp[i % 2] as number + i * v;
    }
    dp[(i + 1) % 2] = m;
  }

  return Math.max(...dp);
};

assertEqual(deleteAndEarn([8, 10, 4, 9, 1, 3, 5, 9, 4, 10]), 37)
assertEqual(deleteAndEarn([3, 1]), 4)
assertEqual(deleteAndEarn([3, 4, 2]), 6)
assertEqual(deleteAndEarn([2, 2, 3, 3, 3, 4]), 9)
