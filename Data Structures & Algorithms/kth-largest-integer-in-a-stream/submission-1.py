class KthLargest:
    nums_heap: List[int]
    k: int

    def __init__(self, k: int, nums: List[int]):
        self.nums_heap = sorted([num for num in nums])
        n = len(self.nums_heap)
        if n >= k:
            self.nums_heap = self.nums_heap[n - k : n]
        heapq.heapify(self.nums_heap)
        self.k = k

    def add(self, val: int) -> int:
        if len(self.nums_heap) >= self.k:
            if val > self.nums_heap[0]:
                heapq.heappop(self.nums_heap)
                heapq.heappush(self.nums_heap, val)
        else:
            heapq.heappush(self.nums_heap, val)
        return self.nums_heap[0]
