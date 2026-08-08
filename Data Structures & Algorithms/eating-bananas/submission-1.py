class Solution:
    def minEatingSpeed(self, piles: List[int], h: int) -> int:
        l, r = 1, max(piles) + 1

        ans = l + r
        while l < r:
            m = (l + r) // 2
            sm = 0
            for pile in piles:
                if pile % m == 0:
                    sm += pile // m
                else:
                    sm += int(pile / m) + 1
                
            if sm <= h:
                ans = min(ans, m)
                r = m
            else:
                l = m + 1
        return ans
