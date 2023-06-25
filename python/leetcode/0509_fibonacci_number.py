from assertion import assert_equal

class Solution:
    """
    F(0) = 0, F(1) = 1
    F(n) = F(n - 1) + F(n - 2), for n > 1.

    O(n), O(2)
    """
    def fib(self, n: int) -> int:
        dp = [0, 1]

        for i in range(2, n+1):
            dp[i % 2] = dp[0] + dp[1]

        return dp[n % 2]
    
if __name__ == '__main__':
    assert_equal(Solution().fib(2), 1)
    assert_equal(Solution().fib(3), 2)
    assert_equal(Solution().fib(4), 3)
    
