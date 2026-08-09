# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def isBalanced(self, root: Optional[TreeNode]) -> bool:
        if root is None:
            return True
        else:
            res = [True]
            self._isBalanced(root, res)
            return res[0]

    def _isBalanced(self, root: Optional[TreeNode], res: List[bool]) -> int:
        if root is None:
            return 0
        else:
            lft = 1 + self._isBalanced(root.left, res)
            rht = 1 + self._isBalanced(root.right, res)
            if abs(lft - rht) > 1:
                res[0] = False
            return max(lft, rht)
 