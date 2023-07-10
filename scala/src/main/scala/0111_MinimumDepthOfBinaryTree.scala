import scala.collection.immutable.Queue

object MinimumDepthOfBinaryTree extends App {
  /*
   * BFS
   * first leaf is the minimum depth.
   *
   * O(logn), O(logn)
   */
  def minDepth(root: TreeNode): Int = {
    def go(nodes: Queue[(TreeNode, Int)]): Int = {
      if (nodes.isEmpty) 0
      else {
        nodes.dequeue match {
          case ((x, l), xs) if x.left == null && x.right == null => l
          case ((x, l), xs) if x.left == null                    => go(xs.enqueue((x.right, l + 1)))
          case ((x, l), xs) if x.right == null                   => go(xs.enqueue((x.left, l + 1)))
          case ((x, l), xs) => go(xs.enqueue((x.left, l + 1)).enqueue((x.right, l + 1)))
        }
      }
    }

    if (root == null) 0 else go(Queue((root, 1)))
  }

  assert(minDepth(TreeNode(1, TreeNode(2, TreeNode.of(4), TreeNode.of(5)), TreeNode.of(3))) == 2)
  assert(minDepth(TreeNode(3, TreeNode.of(9), TreeNode(20, TreeNode.of(15), TreeNode.of(7)))) == 2)
  assert(minDepth(TreeNode(2, null, TreeNode(3, null, TreeNode(4, null, TreeNode(5, null, TreeNode.of(6)))))) == 5)
}
