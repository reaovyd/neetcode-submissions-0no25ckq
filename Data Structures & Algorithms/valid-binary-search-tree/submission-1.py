# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def isValidBST(self, root: Optional[TreeNode]) -> bool:
        lst = []

        def dfs(root: Optional[TreeNode]):
            if root is None:
                return None
            else:
                dfs(root.left)
                lst.append(root.val)
                dfs(root.right)

        dfs(root)
        n = len(lst)
        for i in range(1, n):
            if lst[i] <= lst[i - 1]:
                return False
        return True
        