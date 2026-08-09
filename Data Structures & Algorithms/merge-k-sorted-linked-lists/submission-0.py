# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
    def mergeKLists(self, lists: List[Optional[ListNode]]) -> Optional[ListNode]:
        lst_nums = []
        for lst in lists:
            cur = lst
            while cur:
                lst_nums.append(cur.val)
                cur = cur.next
        lst_nums = sorted(lst_nums)
        dummy = ListNode(-1)
        cur = dummy
        for num in lst_nums:
            cur.next = ListNode(num)
            cur = cur.next
        return dummy.next
