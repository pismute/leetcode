from typing import List
from array import array
from assertion import assert_equal

class Solution:
    """
    [1,100,1,1,1]
                             i=0               i=1
                      1            2       2          3
                 2       3      3    4
                    ...........................

    brute force
    O(2^n), O(n)

    dp: bottom up

    [1,100,1,1,1]
     3 101 2 1 1

    dp[0] = 3

    O(n), O(2)

    dp: top down

    [1,100,1,1,1]
     1,100,2,3,3

    dp[n-1] = 3

    O(n), O(2)
    """
    def minCostClimbingStairs(self, cost: List[int]) -> int:
        dp = array('i', [0, 0])

        for i in range(len(cost)):
            dp[i % 2] = min(dp) + cost[i]

        return min(dp)
    
if __name__ == '__main__':
    assert_equal(Solution().minCostClimbingStairs([1,100,1,1,1]), 3)
    assert_equal(Solution().minCostClimbingStairs([10,15,20]), 15)
    assert_equal(Solution().minCostClimbingStairs([1,100,1,1,1,100,1,1,100,1]), 6)
