class Solution:
    def climbStairs(self, n: int) -> int:
        cur, prev = 0, 1
        nxt = 0
        for i in range(n + 1):
            nxt = cur + prev
            prev = cur
            cur = nxt
        return nxt