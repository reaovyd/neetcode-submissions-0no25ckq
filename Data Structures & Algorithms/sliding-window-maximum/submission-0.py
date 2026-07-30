class HeapItem:
    val: int
    pos: int

    def __init__(self, val, pos):
        self.val = val
        self.pos = pos

    def __lt__(self, other):
        return self.val < other.val

    def __repr__(self) -> str:
        return f"HeapItem: val: {self.val} pos: {self.pos}"


class Solution:
    def maxSlidingWindow(self, nums: List[int], k: int) -> List[int]:
        heap = []
        n = len(nums)
        for i in range(k):
            heapq.heappush(heap, HeapItem(-nums[i], i))
        ans = []
        for i in range(k, n + 1):
            while len(heap) > 0 and (heap[0].pos < (i - k)):
                heapq.heappop(heap)
            ans.append(-heap[0].val)
            if i < n:
                heapq.heappush(heap, HeapItem(-nums[i], i))
        return ans
