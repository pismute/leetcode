from typing import List

class Solution:
    """
    [[0,0,0]
    ,[0,1,0]
    ,[0,0,0]]

    dp: top down

    dp[0][0] = 1
    dp[i][j] = d[i-1][j] + d[i][j-1]
       0  1  2 
    0  1  1  1
    1  1  0  1
    2  1  1  2

    reuse array

    O(n*m), O(m)
    """
    def uniquePathsWithObstacles(self, obstacleGrid: List[List[int]]) -> int:
        n = len(obstacleGrid)
        m = len(obstacleGrid[0])
        dp = [0] * m

        for i in range(n):
            for j in range(m):
                if obstacleGrid[i][j] == 1:
                    dp[j] = 0
                elif i == 0 and j == 0:
                    dp[j] = 1
                elif i == 0:
                    dp[j] = dp[j - 1]
                elif j != 0:
                    dp[j] += dp[j - 1]

        return dp[m - 1]
    
if __name__ == '__main__':
    assert Solution().uniquePathsWithObstacles([[0,0,0],[0,1,0],[0,0,0]]) == 2
    assert Solution().uniquePathsWithObstacles([[0,1],[0,0]]) == 1
    
