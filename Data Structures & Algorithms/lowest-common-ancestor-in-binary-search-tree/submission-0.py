# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def lowestCommonAncestor(
        self, root: TreeNode, p: TreeNode, q: TreeNode
    ) -> TreeNode:
        lst1, lst2 = [], []
        self.searchAndCollect(root, p, lst1)
        self.searchAndCollect(root, q, lst2)
        common = None
        for i in range(min(len(lst1), len(lst2))):
            if lst1[i] == lst2[i]:
                common = lst1[i]
        return common

    def searchAndCollect(
        self, root: Optional[TreeNode], target: TreeNode, lst: List[TreeNode]
    ) -> bool:
        if root is None:
            return False
        else:
            lst.append(root)
            if target.val > root.val:
                return self.searchAndCollect(root.right, target, lst)
            elif target.val < root.val:
                return self.searchAndCollect(root.left, target, lst)
            else:
                return True
