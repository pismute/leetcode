object UniqueBinarySearchTrees extends App {
  /*
    [1, 2, 3]

                1.                     2.                     3
                   [2,     3].      1.    3       [1,     2]
                      3. 2                           2. 1
                       2                1              2




    O(n^n), O(n)

    [1, 2, 3, 4, 5, 6]

                          4.
                [1,2,3]=5    [5, 6] = 2
                    5 * 2 = 10

    numTrees[0] = 1
    numTrees[1] = 1
    numTrees[2] = numTrees[0] * numTrees[1] + numTrees[1] * numTrees[0]
    numTrees[3] = numTrees[0] * numTrees[2] + numTree[1] * numTrees[1] + numTree[2] * numTrees[0]
    ...

    O(n * n), O(n)

   */
  def numTrees(n: Int): Int = {

    // j: root
    def go2(i: Int, j: Int, dp: Array[Int], acc: Int): Int = {
      if (j >= i) acc
      else {
        val l = j
        val r = i - j - 1
        go2(i, j + 1, dp, acc + dp(l) * dp(r))
      }
    }

    // i: nrNodes.
    def go(i: Int, dp: Array[Int]): Int = {
      if (i > n) dp(n)
      else {
        dp(i) = go2(i, 0, dp, 0)

        go(i + 1, dp)
      }
    }

    // 0 = 1
    // 1 = 1
    if (n < 2) 1
    else go(2, Array.fill(n + 1)(1))
  }

  assert(numTrees(3) == 5)
  assert(numTrees(1) == 1)
}
