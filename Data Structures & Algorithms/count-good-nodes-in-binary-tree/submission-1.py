# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def goodNodes(self, root: TreeNode) -> int:
        res = [0]

        def dfs(root: Optional[TreeNode], res: List[int], cur_max: int):
            if root is None:
                return
            else:
                if cur_max <= root.val:
                    res[0] += 1
                cur_max = max(cur_max, root.val)
                dfs(root.left, res, cur_max)
                dfs(root.right, res, cur_max)

        dfs(root, res, -100000)
        return res[0]
