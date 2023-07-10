object UniqueBinarySearchTreesII extends App {
  /*
   *. [1,2,3]
   *
   *      [1                                               2                                       3]
   *.        [2,      3]                             [1]       [3]                [1,         2]
   *           [3]. [2]                                                               2.   1
   *
   *
   * brute force
   * O(n^n), O(n)
   *
   *
   * Cache
   *
   * [], [1], [2], [3], [1, 2], [2, 3], [1, 2, 3]
   * 1   +    n      +      n - 1.   +   n - 2  = n(n+1)/2 => n^2
   *
   * O(n^2), O(n^2)
   * HashMap[subarray] = [subtrees]
   *
   * hm[[]] = [0], 0 = null
   * hm[[1]] = [[0 <- 1 -> 0]]
   * ...
   * hm[[1,2]] = [[ hm[[]] <- 1 -> hm[[2]] ], [ hm[[1]] <- 2 -> hm[[]]] ]]
   * ...
   * hm[[1,2,3]] = [[ hm[[]] <- 1 -> hm[[2,3]]], [ hm[[1]] <- 2 -> hm[[3]]], [hm[[1,2]] <- 3 -> hm[[]] ]]
   *
   * [],                    -> 0 to -1                 -> for i -> j
   * [1], [2], [3].         -> 1 to 1, 2 to 2, 3 to 3. -> ws=1, i = 1 to 3, j = ws to 3
   * [1,2], [2, 3]          -> 1 to 2, 2 to 3          -> ws=2, i = 1 to 3 - ws + 1, j = ws to 3
   * [1,2,3]                -> 1 to 3                  -> ws=3, i = 1 to 3 - ws + 1, j = ws to 3
   *
   * (5 to 0).hashCode == (2 to 0).hashCode
   *
   */
  def generateTrees(n: Int): List[TreeNode] = {
    // i for range start
    // j for range end
    // k for root
    def go2(i: Int, j: Int, k: Int, hm: Map[Range, List[TreeNode]], acc: List[TreeNode]): List[TreeNode] = {
      if (k > j) acc
      else {
        val leftTrees = hm(i until k)
        val rightTrees = hm(k + 1 to j)
        val trees =
          leftTrees.foldLeft(acc) { (ac, left) =>
            rightTrees.foldLeft(ac) { (a, right) =>
              TreeNode(k, left, right) :: a
            }
          }

        go2(i, j, k + 1, hm, trees)
      }
    }

    def go(ws: Int, i: Int, j: Int, hm: Map[Range, List[TreeNode]]): List[TreeNode] = {
      if (ws > n) hm(1 to n)
      else if (j > n) go(ws + 1, 1, ws + 1, hm) // goes to next ws
      else {
        val trees = go2(i, j, i, hm, List.empty)
        go(ws, i + 1, j + 1, hm + ((i to j) -> trees))
      }
    }

    // 1 to 0 for empty
    val emptyRange = 1 to 0
    go(1, 1, 1, Map(emptyRange -> List(null)))
  }

  assert(
    generateTrees(3) ==
      List(
        TreeNode(3, TreeNode(1, null, TreeNode(2, null, null)), null),
        TreeNode(3, TreeNode(2, TreeNode(1, null, null), null), null),
        TreeNode(2, TreeNode(1, null, null), TreeNode(3, null, null)),
        TreeNode(1, null, TreeNode(2, null, TreeNode(3, null, null))),
        TreeNode(1, null, TreeNode(3, TreeNode(2, null, null), null))
      )
  )
  assert(generateTrees(1) == List(TreeNode(1, null, null)))
}
