class WeightedAdjGraph:
    graph: List[List[Tuple[int, int]]]

    def __init__(self, nodes: int):
        self.graph = [[] for _ in range(nodes)]

    def add_edge(self, a: int, b: int, edge_weight: int):
        self.graph[a].append((b, edge_weight))

class Solution:
    def networkDelayTime(self, times: List[List[int]], n: int, k: int) -> int:
        wag = WeightedAdjGraph(n + 1)
        for time in times:
            wag.add_edge(time[0], time[1], time[2])
        MAX = 2 << 31
        dists = [MAX for _ in range(n + 1)]
        dists[k] = 0
        dists[0] = -1

        prio_queue = []
        heapq.heappush(prio_queue, (dists[k], k))

        while prio_queue:
            tup = heapq.heappop(prio_queue)
            dist_to_node, node = tup[0], tup[1]
            if dist_to_node > dists[node]:
                continue
            for neighbor in wag.graph[node]:
                edge_node_y, node_y = neighbor[1], neighbor[0]
                if edge_node_y + dist_to_node < dists[node_y]:
                    dists[node_y] = edge_node_y + dist_to_node
                    heapq.heappush(prio_queue, (dists[node_y], node_y))

        for dist in dists:
            if dist == MAX:
                return -1
        return max(dists)
            
