from typing import List
from array import array

class Solution:
    """
    [1,3,1],
    [1,5,1],
    [4,2,1]]

    dp: topdown
       0  1  2
    0  1  4  5
    1  2  7  6
    2  6  8  7

    O(n*m), (n)
    """
    def minPathSum(self, grid: List[List[int]]) -> int:
        ll = len(grid[0])
        dp = array('i', [0] * ll)

        for i in range(len(grid)):            
            for j in range(0, ll):
                dp[j] = min(dp[j], dp[j-1]) if i > 0 and j > 0 else dp[j] if j == 0 else dp[j - 1]
                dp[j] += grid[i][j]

        return dp[ll - 1]
        
if __name__ == '__main__':
    assert Solution().minPathSum([[1,2],[1,1]]) == 3
    assert Solution().minPathSum([[1,3,1],[1,5,1],[4,2,1]]) == 7
    assert Solution().minPathSum([[1,2,3],[4,5,6]]) == 12
    
