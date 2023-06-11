object ClimbingStairs extends App {
  /*

    n                               0
    1                    1.                    2.
    2                2.         3.      3.             4
    3            3      4     4.   5  4.   5         5    6

    brute force

    O(2^n), O(n)

    caching
    Arrary[stair] = nr of ways.
     [1,2,3]
   3 [2 1 1] 0

    O(n), O(n)


    n
    0. 3
    1. 2
    2. 1
    3  1

    O(n), O(1)
   */
  def climbStairs(n: Int): Int = {
    val s = Array(1, 1)

    for (i <- (n - 2) to 0 by -1) {
      s(i % 2) = s(i % 2) + s((i + 1) % 2)
    }

    s(0)
  }

  assert(climbStairs(2) == 2)
  assert(climbStairs(3) == 3)
}
