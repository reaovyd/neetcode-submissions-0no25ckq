# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def levelOrder(self, root: Optional[TreeNode]) -> List[List[int]]:
        if root is None:
            return []
        q = deque()
        ans = []

        q.append(root)

        while len(q) > 0:
            n = len(q)
            anss = []
            while n > 0:
                node = q.popleft()
                anss.append(node.val)
                if node.left is not None:
                    q.append(node.left)
                if node.right is not None:
                    q.append(node.right)
                n -= 1
            ans.append(anss)

        return ans
 