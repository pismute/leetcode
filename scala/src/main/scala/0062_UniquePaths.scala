object UniquePaths extends App {
  /*
    m = 3, n = 2
                                    0,0
                       (0,1)                    (1, 0)
                 (0, 2)       (1,1)         (1, 1)
                 (1, 2)    (1, 2).        (1, 2)

    brute force,
    O(2^(m+n)), O(m+n)

    dp
    O(m*n), O(m*n)
   */
  def uniquePaths(m: Int, n: Int): Int = {
    val dp = Array.ofDim[Int](n, m)

    dp(n - 1)(m - 1) = 1

    for {
      j <- (0 until n).reverse
      i <- (0 until m).reverse
      if !(i == m - 1 && j == n - 1)
    } {
      val right = if (i == m - 1) 0 else dp(j)(i + 1)
      val down = if (j == n - 1) 0 else dp(j + 1)(i)
      dp(j)(i) = right + down
    }

    dp(0)(0)
  }

  assert(uniquePaths(3, 7) == 28)
  assert(uniquePaths(3, 2) == 3)
}
