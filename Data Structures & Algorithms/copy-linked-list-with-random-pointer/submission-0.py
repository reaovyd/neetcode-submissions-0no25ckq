"""
# Definition for a Node.
class Node:
    def __init__(self, x: int, next: 'Node' = None, random: 'Node' = None):
        self.val = int(x)
        self.next = next
        self.random = random
"""

class Solution:
    def copyRandomList(self, head: "Optional[Node]") -> "Optional[Node]":
        if head is None:
            return None
        cur = head
        mp = {}
        lst = []
        idx = 0
        while cur:
            mp[cur] = idx
            lst.append((cur, Node(cur.val)))
            idx += 1
            cur = cur.next
        for i in range(len(lst)):
            cur, copy = lst[i][0], lst[i][1]
            if cur.random is None:
                copy.random = None
            else:
                copy.random = lst[mp[cur.random]][1]
            if cur.next is None:
                copy.next = None
            else:
                copy.next = lst[mp[cur.next]][1]
        return lst[0][1]
 