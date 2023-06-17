from array import array

class Solution:
    """
    m = 3, n = 2
    
    brute force:
                                          (0, 0)
                                (0, 1)               (0, 1)

    O(2^(m+n)), O(m+n)

    dp: bot
    dp: top down
    
       0  1   2
    0  
    1     1   1
    2     1   2
    3     1   3

    O(m*n), (min(m, n))
    """
    def uniquePaths(self, m: int, n: int) -> int:
        mi = min(m, n)
        ma = max(m, n)
        dp = [0] * (mi + 1)

        for i in range(ma):
            for j in range(1, mi + 1):
                if i == 0 and j == 1:
                    dp[j] = 1
                else:
                    dp[j] += dp[j-1]
                
        return dp[mi]
        
if __name__ == '__main__':
    assert Solution().uniquePaths(3, 2) == 3
    assert Solution().uniquePaths(3, 7) == 28
    
