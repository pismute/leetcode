from typing import List
from assertion import assert_equal
from functools import cache

class Solution:
    """
    """
    def permute(self, nums: List[int]) -> List[List[int]]:
        nums_mask = (1 << len(nums)) - 1

        def go(mask: int) -> List[List[int]]:
            if mask == nums_mask:
                return [[]]
            else:
                all = []
                     
                for j in range(len(nums)):
                    if mask & (1 << j) == 0:
                        res = go(mask | (1 << j))
                        for arr in res:
                            arr.append(nums[j])
                     
                        all.extend(res)

                return all

        return go(0)
                     
if __name__ == '__main__':
    assert_equal(Solution().permute([1]), [[1]])
    assert_equal(Solution().permute([0,1]), [[0,1],[1,0]])
    assert_equal(Solution().permute([1,2,3]), [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]])
    
