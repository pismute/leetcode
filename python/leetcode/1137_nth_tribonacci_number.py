class Solution:
    """
    T0 = 0, T1 = 1, T2 = 1, and Tn+3 = Tn + Tn+1 + Tn+2 for n >= 0.

    O(n), O(3)
    """
    def tribonacci(self, n: int) -> int:
        dp = [0, 1, 1]

        for i in range(3, n + 1):
            dp[i % 3] = sum(dp)

        return dp[n % 3]
        
if __name__ == '__main__':
    assert Solution().tribonacci(4) == 4
    assert Solution().tribonacci(25) == 1389537
    
