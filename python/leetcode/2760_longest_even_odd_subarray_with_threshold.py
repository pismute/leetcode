from typing import List
from assertion import assert_equal

class Solution:
    """
    """
    def longestAlternatingSubarray(self, nums: List[int], threshold: int) -> int:
        res = 0
        cnt = 0

        for i in range(len(nums)):
            if nums[i] <= threshold:
                if cnt > 0 and nums[i] % 2 != nums[i - 1] % 2:
                    cnt += 1
                elif nums[i] % 2 == 0:
                    cnt = 1
                else:
                    cnt = 0
                
            else:
                cnt = 0

            res = max(res, cnt)
                
        return res


if __name__ == '__main__':
    
    assert_equal(Solution().longestAlternatingSubarray([2,3,3,10], 18), 2)
    assert_equal(Solution().longestAlternatingSubarray([4,10,3], 10), 2)
    assert_equal(Solution().longestAlternatingSubarray([8,4], 6), 1)
    assert_equal(Solution().longestAlternatingSubarray([2,10,5], 7), 1)
    assert_equal(Solution().longestAlternatingSubarray([1], 1), 0)
    assert_equal(Solution().longestAlternatingSubarray([3,2,5,4], 5), 3)
    assert_equal(Solution().longestAlternatingSubarray([1,2], 2), 1)
    assert_equal(Solution().longestAlternatingSubarray([2,3,4,5], 4), 3)
