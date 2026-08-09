# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        ln = 0
        cur = head
        while cur:
            cur = cur.next
            ln += 1
        prev = None
        cur = head
        for i in range(ln - n):
            prev = cur
            cur = cur.next
        if prev is None:
            head = head.next
            return head
        else:
            prev.next = cur.next
            return head
