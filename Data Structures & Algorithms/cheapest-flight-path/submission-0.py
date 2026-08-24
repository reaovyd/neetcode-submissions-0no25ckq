class AdjGraph:
    graph: List[List[int]]

    def __init__(self, n: int):
        self.graph = [[] for _ in range(n)]
    
    def add_edge(self, a: int, b: int, edge_weight: int):
        self.graph[a].append((b, edge_weight))

class Solution:
    def findCheapestPrice(self, n: int, flights: List[List[int]], src: int, dst: int, k: int) -> int:
        ag = AdjGraph(n)
        max_edges = k + 1
        for flight in flights:
            ag.add_edge(flight[0], flight[1], flight[2])
        dp = [[float('inf') for _ in range(n)] for _ in range(max_edges + 1)]
        for i in range(max_edges + 1):
            dp[i][src] = 0

        prio_queue = []
        heapq.heappush(prio_queue, (dp[0][src], src, 0))

        while prio_queue:
            dist, u, edges = heapq.heappop(prio_queue)
            if edges >= max_edges or dist > dp[edges][u]:
                continue
            for v, w in ag.graph[u]:
                nd = w + dp[edges][u]
                if nd < dp[edges + 1][v]:
                    dp[edges + 1][v] = nd
                    heapq.heappush(prio_queue, (nd, v, edges + 1))

        ans = float('inf')
        for i in range(max_edges + 1):
            ans = min(ans, dp[i][dst])
        return ans if ans != float('inf') else -1
