from typing import List
from assertion import assert_equal
from collections import Counter

class Solution:
    """
    """
    def singleNumber(self, nums: List[int]) -> int:
        cnt = Counter(nums)

        for k, v in cnt.items():
            if v == 1:
                return k

        # impossible

if __name__ == '__main__':
    assert_equal(Solution().singleNumber([2,2,3,2]), 3)
    assert_equal(Solution().singleNumber([0,1,0,1,0,1,99]), 99)
    

