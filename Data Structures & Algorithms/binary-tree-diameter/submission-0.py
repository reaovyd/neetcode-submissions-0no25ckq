# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def diameterOfBinaryTree(self, root: Optional[TreeNode]) -> int:
        self.ans = 0
        self._diameterOfBinaryTree(root)
        return self.ans

    def _diameterOfBinaryTree(self, root: Optional[TreeNode]) -> int:
        if root is None:
            return -1
        else:
            lft = 1 + self._diameterOfBinaryTree(root.left)
            rht = 1 + self._diameterOfBinaryTree(root.right)
            self.ans = max(self.ans, lft + rht)
            return max(lft, rht)
