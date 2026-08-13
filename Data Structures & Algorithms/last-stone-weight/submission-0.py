class Solution:
    def lastStoneWeight(self, stones: List[int]) -> int:
        stones = [-stone for stone in stones]
        heapq.heapify(stones)
        while True:
            n = len(stones)
            if n == 0:
                return 0
            elif n == 1:
                return -stones[0]
            else:
                a = -heapq.heappop(stones)
                b = -heapq.heappop(stones)
                x, y = min(a, b), max(a, b)
                if x != y:
                    heapq.heappush(stones, -(y - x))
