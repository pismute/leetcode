from typing import List
from assertion import assert_equal
from functools import cache

class Solution:
    """
    """
    def specialPerm(self, nums: List[int]) -> int:
        MOD = 10**9 + 7
        @cache
        def go(mask: int, cur: int) -> int:
            if mask == ((1 << len(nums)) -1):
                return 1
            else:
                sum = 0
                for i in range(len(nums)):
                    if (mask & 1 << i == 0 and
                        (cur < 0 or
                         nums[cur] % nums[i] == 0 or
                         nums[i] % nums[cur] == 0)):
                        sum += go(mask | 1 << i, i)

                return sum

        return go(0, -1) % MOD

if __name__ == '__main__':
    assert_equal(Solution().specialPerm([1,2,4,8,16,32,64,128,256,512,1024,2048,4096,8192]), 178290591)
    assert_equal(Solution().specialPerm([2,3,6]), 2)
    assert_equal(Solution().specialPerm([1,4,3]), 2)
