from typing import List
from assertion import assert_equal

class Solution:
    """
    "leetcode", ["leet","code"]

                                    leet               code
                             leet          code
    length of leetcode:n
    length of dict: m
    matching string = n
    
    O(n*m*n), O(n)

    "leetcode"
    [fffffffft]
     t   t
    
    i=0:  leet 
    """
    def wordBreak(self, s: str, wordDict: List[str]) -> bool:
        dp = [False] * (len(s) + 1)
        dp[-1] = True
        for i in range(len(s) - 1, -1, -1):
            for w in wordDict:
                if (i + len(w)) <= len(s) and w == s[i:i+len(w)]:
                    dp[i] = dp[i + len(w)]
                if dp[i]:
                    break

        return dp[0]

if __name__ == '__main__':
    assert_equal(Solution().wordBreak("aaaaaaa", ["aaaa","aaa"]), True)
    assert_equal(Solution().wordBreak("leetcode", ["leet","code"]), True)
    assert_equal(Solution().wordBreak("applepenapple", ["apple","pen"]), True)
    assert_equal(Solution().wordBreak("catsandog", ["cats","dog","sand","and","cat"]), False)
