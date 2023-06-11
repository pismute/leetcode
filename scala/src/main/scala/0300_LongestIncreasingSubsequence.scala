object LongestIncreasingSubsequence extends App {
  /*
    [0,1,0,3,2,3]

    [0, 1, 0, 3]

               0           1       0.    3
          1.    _.  3.   _.  3
        _. 3.   3.      3
      3.


    brute force.
    O(2^n), O(n)

    Cache

    Array(index) = length of LIS
i     0 1 2 3 4 5
input[0,1,0,3,2,3]
    a(5)                                 = 1
    a(4) = max(1, a(5)+1)                = 2
    a(3) = max(1, a(4)+1, a(5)+1)        = 1
    a(2) = max(1, a(3)+1, a(4)+1, a(5)+1) = max(1, 2, 3, 2) = 3
    a(1) = max(1, a(3)+1, a(4)+1, a(5)+1) = max(1, 2, 3, 2) = 3
    a(0) = max(1, ....)                  = max(1, 4, ...)
    4

    O(n *n = n^2), O(n)
   */
  def lengthOfLIS(nums: Array[Int]): Int = {

    def go(i: Int, max: Int, a: Array[Int]): Int = {
      if (i < 0) max
      else {
        val v = nums(i)

        a(i) = (i until nums.length)
          .filter(j => nums(j) > v)
          .map(j => a(j))
          .maxOption
          .getOrElse(0) + 1

        go(i - 1, max.max(a(i)), a)
      }
    }

    go(nums.length - 2, 1, Array.fill(nums.length)(1))
  }

  assert(lengthOfLIS(Array(10, 9, 2, 5, 3, 7, 101, 18)) == 4)
  assert(lengthOfLIS(Array(0, 1, 0, 3, 2, 3)) == 4)
  assert(lengthOfLIS(Array(7, 7, 7, 7, 7, 7, 7)) == 1)
}
