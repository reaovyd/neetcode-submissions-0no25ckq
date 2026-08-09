# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def isSubtree(self, root: Optional[TreeNode], subRoot: Optional[TreeNode]) -> bool:
        return self._isSubtree(root, subRoot)

    def _isSubtree(self, root: Optional[TreeNode], subRoot: Optional[TreeNode]) -> bool:
        if root is None:
            return False
        else:
            if self.isSameTree(root, subRoot):
                return True
            else:
                return self._isSubtree(root.left, subRoot) or self._isSubtree(
                    root.right, subRoot
                )

    def isSameTree(self, p: Optional[TreeNode], q: Optional[TreeNode]) -> bool:
        if p is None and q is not None:
            return False
        elif p is not None and q is None:
            return False
        elif p is None and q is None:
            return True
        elif p is not None and q is not None:
            if p.val != q.val:
                return False
            else:
                return self.isSameTree(p.left, q.left) and self.isSameTree(
                    p.right, q.right
                )
        else:
            return True
