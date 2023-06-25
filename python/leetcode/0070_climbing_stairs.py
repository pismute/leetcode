from assertion import assert_equal

class Solution:
    """
    n = 4
    [1,2,3,4]
                    1                       2
             2            3           3          4
         3      4                 4
      4

    brute force
    O(2^n), O(n)

    memoization
    O(2n), O(n + n)

    dp: bottom up
    [1,2,3,4]
     0,1,2,3 - index
    [3 2 1 1] - answer 3
    
    O(n), O(2)

    dp: top down
     0 1 2 3
    [1,1,2,3] - anwser 3
    O(n), O(2)
    
    """
    def climbStairs(self, n: int) -> int:
        dp = [1, 1]

        for i in range(2, n+1):
            dp[i % 2] = dp[0] + dp[1]

        return dp[n % 2]

if __name__ == '__main__':
    assert_equal(Solution().climbStairs(2), 2)
    assert_equal(Solution().climbStairs(3), 3)
