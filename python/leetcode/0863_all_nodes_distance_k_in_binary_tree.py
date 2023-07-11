from typing import List
from collections import deque, defaultdict
from assertion import assert_equal

class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None

    def __str__(self) -> str:
        return f"TreeNode({self.val}, {self.left}, {self.right})"
        
class Solution:
    """
    [3,5,1,6,2,0,8,null,null,7,4]

    bfs:

    building adjacent graph: O(n), O(n)
    finding distance: O(n), O(n)
    
    O(2n), O(2n)
    """
    def distanceK(self, root: TreeNode, target: TreeNode, k: int) -> List[int]:
        graph = defaultdict(list)

        q = deque()
        q.append(root)

        while q:
            cur = q.popleft()
            if cur:
                if cur.left:
                    graph[cur.val].append(cur.left.val)
                    graph[cur.left.val].append(cur.val)
                    q.append(cur.left)
                if cur.right:
                    graph[cur.val].append(cur.right.val)
                    graph[cur.right.val].append(cur.val)
                    q.append(cur.right)

        res = []
        visited = set()
        q2 = deque()
        q2.append((target.val, 0))
        while q2:
            (cur, d) = q2.popleft()
            if cur in visited:
                continue
            elif d == k:
                res.append(cur)
            elif cur in graph:
                for v in graph[cur]:
                    q2.append((v, d + 1))
            visited.add(cur)

        return res
        
if __name__ == '__main__':
    # [0,1,3,null,2]
    t = TreeNode(0)
    t.left = TreeNode(1)
    t.right = TreeNode(3)
    t.left.right = TreeNode(2)
    assert_equal(Solution().distanceK(t, TreeNode(1), 2), [3])
    # [3,5,1,6,2,0,8,null,null,7,4]
    t = TreeNode(3)
    t.left = TreeNode(5)
    t.right = TreeNode(1)
    t.left.left = TreeNode(6)
    t.left.right = TreeNode(2)
    t.right.left = TreeNode(0)
    t.right.right = TreeNode(8)
    t.left.right.left = TreeNode(7)
    t.left.right.right = TreeNode(4)
    assert_equal(Solution().distanceK(t, TreeNode(5), 2), [7,4,1])
    assert_equal(Solution().distanceK(TreeNode(1), TreeNode(1), 3), [])
    
