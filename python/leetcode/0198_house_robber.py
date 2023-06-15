from typing import List
from array import array

class Solution:
    """
    [1,2,3,1]

                       1           2
                 3              1

    brute force
    O(2^n), O(n)

    dp: bottom up

    dp[3] = 1
    dp[2] = 3
    dp[1] = 2 + max(dp[1+2:]) = 3
    dp[0] = 1 + max(dp[0+2:]) = 4

    max(dp[:2]) = 4
    
    O(n), O(n)

    looking back to i+2 position's subarray.
    
    O(n), O(1)

    dp: top down

    dp[0] = 1
    dp[1] = 2
    dp[2] = 3 + max(dp[:2-2 + 1(for exclusion)]) = 4
    dp[3] = 1 + max(dp[:3-2 + 1(for exclusion)]) = 3

    max(dp[4-2:]) = 4
    
    O(n), O(n)

    looking back to i-2 position's subarray.
    
    O(n), O(1)
    
    """
    def rob(self, nums: List[int]) -> int:
        dp = array('i', [0,0])

        for i, v in enumerate(nums):
            m = max(dp)
            dp[i % 2] = v + dp[i % 2]
            dp[(i+1) % 2] = m # keep maximum value.

        return max(dp)

if __name__ == '__main__':
    assert Solution().rob([1,2,3,1]) == 4
    assert Solution().rob([2,7,9,3,1]) == 12
    
