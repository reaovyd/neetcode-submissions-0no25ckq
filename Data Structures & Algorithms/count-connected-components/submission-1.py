class AdjGraph:
    graph: List[Set[int]]

    def __init__(self, n: int):
        self.graph = [set() for _ in range(n)]
    
    def add_edge(self, a: int, b: int):
        self.graph[a].add(b)
        self.graph[b].add(a)

class Solution:
    def countComponents(self, n: int, edges: List[List[int]]) -> int:
        ag = AdjGraph(n)
        visited = [False for _ in range(n)]
        for edge in edges:
            ag.add_edge(edge[0], edge[1])

        def dfs(node: int):
            visited[node] = True
            for n in ag.graph[node]:
                if not visited[n]:
                    dfs(n)

        ans = 0
        for i in range(n):
            if not visited[i]:
                dfs(i)
                ans += 1
        return ans