from typing import List
from assertion import assert_equal
import itertools
import functools
import heapq

class Solution:
    """
    [1,3,5,1], k = 2

    j in k # k - 1, cuts
    score is:
    W[0] + W[j1 - 1]
    W[j1] + W[j2 - 1]
    W[j2] + W[j3 - 1]
    ...
    W[jk] + W[n - 1]

    =:
    W[0] + (W[j1 - 1] + W[j1]) + (W[j2 - 1] + W[j2]) ... + W[n - 1]

    =:
    W[0] + PW[j1] + PW[j2] ... + W[n - 1]

    k = 3
    min[i] = W[0] + PW[i1] + PW[i2] + W[3]
    max[j] = W[0] + PW[j1] + PW[j2] + W[3]
    max[j] - min[i] = PW[j1] + PW[j2] - (PW[i1] + PW[i2])
    
    O(nk), O(k)
    """
    def putMarbles(self, weights: List[int], k: int) -> int:
        sums = [a + b for a, b in itertools.pairwise(weights)]
        return sum(heapq.nlargest(k-1, sums)) - sum(heapq.nsmallest(k-1, sums))

if __name__ == '__main__':
    print("-------------------------")
    assert_equal(Solution().putMarbles([1,3,5,1], 2), 4)
    # assert_equal(Solution().putMarbles([1,3,5,1], 3), 4)
    # assert_equal(Solution().putMarbles([1, 3], 2), 0)
    # assert_equal(Solution().putMarbles([54,6,34,66,63], 4), 89)
    # assert_equal(Solution().putMarbles(
    #     [54,6,34,66,63,52,39,62,46,75,28,65,18,37,18,13,33,69,19,40,13,10,43,61,72], 4),
    #              289)
    # assert_equal(Solution().putMarbles(
    #     [46,37,46,17,40,50,54,11,1,25,43,21,31,29,58,49,73,54,5,52,73,54,6,22,58,9,34,
    #      21,58,68,63], 30), 119)


