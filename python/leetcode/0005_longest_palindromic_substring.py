class Solution:
    """
    "babad"
    isPalindrome: n
    moving center: n
    O(n*n), O(1)
    """
    def longestPalindrome(self, s: str) -> str:
        def maxs(s1: str, s2: str) -> str:
            return s1 if len(s1) >= len(s2) else s2
            
        def getPalindrome(i: int, j: int) -> str:
            if i < 0 or j >= len(s) or s[i] != s[j]:
                return s[i+1: j]
            else:
                return getPalindrome(i-1, j+1)

        longest = ""
        for i in range(len(s)):
            longest = maxs(maxs(longest, getPalindrome(i, i)), getPalindrome(i, i+1))

        return longest

if __name__ == '__main__':
    assert Solution().longestPalindrome("babad") == "bab"
    assert Solution().longestPalindrome("cbbd") == "bb"
    
