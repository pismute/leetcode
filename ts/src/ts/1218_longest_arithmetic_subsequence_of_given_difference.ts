import assertEqual from './assert';

/*
 *
 *
 * O(n), O(n)
 */
function longestSubsequence(arr: number[], difference: number): number {
  const cache = new Map<number, number>();

  let longest = 0;

  for (let x of arr) {
    let nr = 1 + (cache.get(x - difference) || 0);

    cache.set(x, nr);

    longest = Math.max(longest, nr);
  }

  return longest;
};

assertEqual(longestSubsequence([1, 2, 3, 4], 1), 4)
assertEqual(longestSubsequence([1, 3, 5, 7], 1), 1)
assertEqual(longestSubsequence([1, 5, 7, 8, 5, 3, 4, 2, 1], -2), 4)
assertEqual(longestSubsequence([3, 4, -3, -2, -4], -5), 2)




