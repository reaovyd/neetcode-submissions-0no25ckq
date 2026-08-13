class KthLargest:
    nums_heap: List[int]
    k: int

    def __init__(self, k: int, nums: List[int]):
        self.nums_heap = [-num for num in nums]
        heapq.heapify(self.nums_heap)
        self.k = k

    def add(self, val: int) -> int:
        heapq.heappush(self.nums_heap, -val)
        kk = self.k
        lst = []
        while kk > 0:
            lst.append(heapq.heappop(self.nums_heap))
            kk -= 1
        val = lst[-1]
        kk = len(lst)
        while kk > 0:
            heapq.heappush(self.nums_heap, lst[kk - 1])
            kk -= 1
        return -val
