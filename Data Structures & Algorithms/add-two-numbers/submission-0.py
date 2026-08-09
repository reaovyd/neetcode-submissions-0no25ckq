# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
    def addTwoNumbers(
        self, l1: Optional[ListNode], l2: Optional[ListNode]
    ) -> Optional[ListNode]:
        carry = 0
        dummy = ListNode(-1)
        cur = dummy
        while l1 and l2:
            sm = l1.val + l2.val + carry
            val = sm % 10
            carry = sm // 10 % 10
            cur.next = ListNode(val)
            cur = cur.next
            l1 = l1.next
            l2 = l2.next
        while l1:
            sm = l1.val + carry
            val = sm % 10
            carry = sm // 10 % 10
            cur.next = ListNode(val)
            cur = cur.next
            l1 = l1.next
        while l2:
            sm = l2.val + carry
            val = sm % 10
            carry = sm // 10 % 10
            cur.next = ListNode(val)
            cur = cur.next
            l2 = l2.next
        if carry != 0:
            cur.next = ListNode(carry)

        return dummy.next
 