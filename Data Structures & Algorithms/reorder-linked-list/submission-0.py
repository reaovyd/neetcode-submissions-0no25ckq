class Solution:
    def reorderList(self, head: Optional[ListNode]) -> None:
        def reverse(head: Optional[ListNode]) -> ListNode:
            next, prev = None, None
            while head:
                next = head.next
                head.next = prev
                prev = head
                head = next
            return prev

        ln = 0
        ptr = head
        while ptr:
            ln += 1
            ptr = ptr.next
        if ln == 1:
            return None
        half = head
        other_half = head
        for _ in range(ln // 2):
            other_half = other_half.next
        c1 = half
        while half and half.next != other_half:
            half = half.next
        half.next = None
        c2 = reverse(other_half)
        n1, n2 = None, None
        while c1 and c2:
            n1 = c1.next
            n2 = c2.next
            c1.next = c2
            if not n1:
                c2.next = n2
            else:
                c2.next = n1
            c1 = n1
            c2 = n2
