class Solution:
    def leastInterval(self, tasks: list[str], n: int) -> int:
        time = 0
        pq = [-count for count in Counter(tasks).values()]
        heapq.heapify(pq)
        q = deque()
        while pq or q:
            if not pq:
                time = q[0][0]
            while q and q[0][0] <= time:
                heapq.heappush(pq, q.popleft()[1])
            cnt = 1 + heapq.heappop(pq)
            time += 1
            if cnt:
                q.append((time + n, cnt))
        return time

        
