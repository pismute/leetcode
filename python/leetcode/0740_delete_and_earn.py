from typing import List
from array import array
import sys
from collections import defaultdict

class Solution:
    """
    [2,2,3,3,3,4]

    count: dict 2 -> 2, 3 -> 3, 4 -> 1

    [2 3 4]
                               2;[4]             3;[]                  ;[]
                               4

    count: O(n), O(1)
    dfs: O(2^n), O(n)

    dp: top down
    [2,     3,     4]
     2*2  3*3  4*1 + 2*2 =  9
    
    """
    def deleteAndEarn(self, nums: List[int]) -> int:
        start = sys.maxsize
        end = -sys.maxsize
        count = defaultdict(int)

        for v in nums:
            count[v] += 1
            start = min(start, v)
            end = max(end, v)

        dp = [0, 0]
        
        for i in range(start, end + 1):
            m = max(dp)
            dp[i % 2] = dp[i%2] + count[i] * i
            dp[(i+1) % 2] = m

        return max(dp)


if __name__ == '__main__':
    assert Solution().deleteAndEarn([3,4,2]) == 6
    assert Solution().deleteAndEarn([2,2,3,3,3,4]) == 9
    
