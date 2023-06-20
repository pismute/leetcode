from typing import List
from array import array

class Solution:
    """
    [2],
    [3,4],
    [6,5,7],
    [4,1,8,3]

    dp:
    base: [4,1,8,3]
    1st: [6 + min(4, 1), 5 + min(1, 8), 7 + min(8, 3)]
    ....

    dp[j] = min[dp[i], dp[i+1]) + triangle[i][j]

    O(n!), O(n)
    """
    def minimumTotal(self, triangle: List[List[int]]) -> int:
        dp = array('i', triangle[-1][:])

        for i in reversed(range(len(triangle) - 1)):
            for j in range(0, i + 1):
                dp[j] = triangle[i][j] + min(dp[j], dp[j+1])

        return dp[0]

if __name__ == '__main__':
    assert Solution().minimumTotal([[2],[3,4],[6,5,7],[4,1,8,3]]) == 11
    assert Solution().minimumTotal([[-10]]) == -10
    
