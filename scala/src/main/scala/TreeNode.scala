case class TreeNode(value: Int, left: TreeNode, right: TreeNode)

object TreeNode {
  def of(v: Int): TreeNode = TreeNode(v, null, null)
}
