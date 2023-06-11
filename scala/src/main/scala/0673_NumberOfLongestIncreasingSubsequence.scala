object NumberOfLongestIncreasingSubsequence extends App {
  /*
    [1,3,5,4,7]

    O(2^n), O(n)

            1;     3      5        4     7
         3;    5 4 7
     5;  4; 7. 7.
     7   7

    cache
     0 1 2 3 4 - index
    [1,3,5,4,7]
    [4,3,2,2,1]
    [2,2,1,1,1]
    array[index = (max length of subseuence, count)

                                                       maxLen
    a[4] =  (1, 1)                                        1
    a[3] = (max(1, a[4]._1 + 1), sum(a[4]._2)) = (2, 1);  2
    a[2] = (max(1, a[4]._1 + 1), sum(a[4]._2)) = (2, 1);  2
    a[1] = (max(1, a[2] + 1, a[3] + 1, a[4] + 1),
            sum(a[2]._2 + a[2]._2))            = (3, 2);  3
    a[0] = (max(1, a[1] + 1,), sum(a[1]._2) = (4, 2);     4

    O(n * n), O(n)
   */
  def findNumberOfLIS(nums: Array[Int]): Int = {
    def go2(i: Int, j: Int, maxLen: Int, count: Int, a: Array[(Int, Int)]): (Int, Int) = {
      if (j >= nums.length) (maxLen, count)
      else {
        if (nums(i) < nums(j)) {
          val (len, c) = a(j)
          go2(
            i,
            j + 1,
            maxLen.max(len + 1),
            if (len + 1 > maxLen) c else if (len + 1 == maxLen) c + count else count,
            a
          )
        } else go2(i, j + 1, maxLen, count, a)
      }
    }

    def go(i: Int, maxLen: Int, count: Int, a: Array[(Int, Int)]): Int = {
      if (i < 0) count
      else {
        a(i) = go2(i, i + 1, 1, 1, a)

        val (len, c) = a(i)

        go(i - 1, maxLen.max(len), if (len > maxLen) c else if (len == maxLen) count + c else count, a)
      }
    }

    go(nums.length - 2, 1, 1, Array.fill(nums.length)(1 -> 1))
  }

  assert(findNumberOfLIS(Array(1, 3, 5, 4, 7)) == 2)
  assert(findNumberOfLIS(Array(2, 2, 2, 2, 2)) == 5)

}
