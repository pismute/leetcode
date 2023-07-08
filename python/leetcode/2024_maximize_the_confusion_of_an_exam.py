from typing import List
from assertion import assert_equal

class Solution:
    """
    ti

    T
    T
    F: if k >0: k -=1 else: ma = max(ma, ti - i + 1), 
    F

    O(2n), (1)
    """
    def maxConsecutiveAnswers(self, answerKey: str, k: int) -> int:
        ti = 0
        fi = 0
        tk = k
        fk = k
        ma = 0
        for i in range(len(answerKey)):
            if answerKey[i] == 'F':
                if tk > 0:
                    tk -= 1
                else:
                    ma = max(ma, i - ti)
                    while answerKey[ti] == 'T':
                        ti += 1
                    ti += 1
            else:
                if fk > 0:
                    fk -= 1
                else:
                    ma = max(ma, i - fi)
                    while answerKey[fi] == 'F':
                        fi += 1
                    fi += 1

        return max(ma, len(answerKey) - ti, len(answerKey) - fi)

if __name__ == '__main__':
    assert_equal(Solution().maxConsecutiveAnswers("TTTFFTFFTF", 3), 8)
    assert_equal(Solution().maxConsecutiveAnswers("FFFTTFTTFT", 3), 8)
    assert_equal(Solution().maxConsecutiveAnswers("TTFF", 2), 4)
    assert_equal(Solution().maxConsecutiveAnswers("TFFT", 1), 3)
    assert_equal(Solution().maxConsecutiveAnswers("TTFTTFTT", 1), 5)
