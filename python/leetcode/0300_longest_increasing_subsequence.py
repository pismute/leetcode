from assertion import assert_equal
from typing import List

class Solution:
    """
    [0,1,0,3,2,3]
    
    dp:
    [1,2,1,3,3,4]
    
    O(n^2), O(n)
    """
    def lengthOfLIS(self, nums: List[int]) -> int:
        dp = [1] * len(nums)

        ma = 0
        for i in range(len(nums)):
            for j in range(i + 1):
                if nums[j] < nums[i]:
                    dp[i] = max(dp[i], dp[j] + 1)
            ma = max(ma, dp[i])
            
        return ma
        
if __name__ == '__main__':
    assert_equal(Solution().lengthOfLIS([10,9,2,5,3,7,101,18]), 4)
    assert_equal(Solution().lengthOfLIS([0,1,0,3,2,3]), 4)
    assert_equal(Solution().lengthOfLIS([7,7,7,7,7,7,7]), 1)
    
