from typing import List
from assertion import assert_equal

class Solution:
    """
    "horse"
    "ros"

        i
    horse
    ros
       j
    
    ==: (i+1, j+1)
    insert: (i, j+1)
    delete: (i+1, j)
    replace: (i+1, j+1)

    dp[i][j] = dp[i-1][j-1] if word1[i] == word2[j] else min(dp[i][j-1], dp[i-1][j], dp[i-1][j-1]) + 1
    
      " r o s
    " 0 1 2 3    
    h 1 1 2 3
    o 2 2 1 2
    r 3 2 2 2
    s 4 3 3 2
    e 5 4 4 3

    O(n*m), O(min(n, m))
    """
    def minDistance(self, word1: str, word2: str) -> int:
        dp = [[0] * (len(word2) + 1) for _ in range(0, 2)]

        # base case
        for j in range(1, len(word2) + 1):
            dp[1][j] = j
            
        for i in range(len(word1)):
            dp[i % 2][0] = i + 1 # base case
            for j in range(1, len(word2) + 1):
                dp[i % 2][j] = (dp[(i+1) % 2][j-1] if word1[i] == word2[j - 1]
                                else min(dp[i % 2][j-1], dp[(i+1) % 2][j], dp[(i+1) % 2][j-1]) + 1)

        return dp[(len(word1) + 1) % 2][len(word2)]

if __name__ == '__main__':
    assert_equal(Solution().minDistance("horse", "ros"), 3)
    assert_equal(Solution().minDistance("intention", "execution"), 5)
    

