class Solution:
    def findKthLargest(self, nums: List[int], k: int) -> int:
        heap = []
        for num in nums:
            if len(heap) >= k:
                if num > heap[0]:
                    heapq.heappop(heap)
                    heapq.heappush(heap, num)
            else:
                heapq.heappush(heap, num)
        return heap[0]
