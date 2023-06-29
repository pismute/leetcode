from assertion import assert_equal

class Solution:
    """
    "cbbd"

    i
    cbbd
    dbbc
    j

    ==: (i+1, j+1)  =  +1
    !=: (i+1, j).max(i, j+1) = 0
    
      " c b b d
    " 0 0 0 0 0
    d 0 0 0 0 1
    b 0 0 1 1 1
    b 0 0 1 2 2
    c 0 1 1 2 2

    O(n*n), O(2n)
    """
    def longestPalindromeSubseq(self, s: str) -> int:
        dp = [[0] * (len(s) + 1) for _ in range(2)]

        for i in range(len(s) - 1, -1, -1):
            for j in range(1, len(s) + 1):
                if s[i] == s[j - 1]:
                    dp[i % 2][j] = dp[(i+1) %2][j - 1] + 1
                else:
                    dp[i % 2][j] = max(dp[i % 2][j - 1], dp[(i+1)%2][j])

        return dp[i % 2][len(s)]

if __name__ == '__main__':
    assert_equal(Solution().longestPalindromeSubseq("cbbd"), 2)
    assert_equal(Solution().longestPalindromeSubseq("bbbab"), 4)
    

