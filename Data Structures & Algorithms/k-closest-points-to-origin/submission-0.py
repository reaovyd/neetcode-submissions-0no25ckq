from math import sqrt

class HeapValue:
    point: List[int]
    dist: float

    def __init__(self, point: List[int], dist: float):
        self.point = point
        self.dist = dist

    def __lt__(self, other: HeapValue) -> bool:
        return self.dist < other.dist


class Solution:
    def kClosest(self, points: List[List[int]], k: int) -> List[List[int]]:
        heap = []
        for point in points:
            x_dist = point[0]
            y_dist = point[1]
            x_dist *= x_dist
            y_dist *= y_dist
            dist = sqrt(x_dist + y_dist)
            if len(heap) >= k:
                if -dist > heap[0].dist:
                    heapq.heappop(heap)
                    heapq.heappush(heap, HeapValue(point, -dist))
            else:
                heapq.heappush(heap, HeapValue(point, -dist))

        return [val.point for val in heap]