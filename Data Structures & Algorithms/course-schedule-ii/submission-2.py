class AdjGraph:
    adj_graph: List[Set[int]]
    indegrees: List[int]

    def __init__(self, n: int):
        self.adj_graph = [set() for _ in range(n)]
        self.indegrees = [0 for _ in range(n)]
    
    def add_edge(self, a: int, b: int):
        self.adj_graph[a].add(b)
        self.indegrees[b] += 1

class Solution:
    def findOrder(self, numCourses: int, prerequisites: List[List[int]]) -> List[int]:
        ag = AdjGraph(numCourses)

        
        for edge in prerequisites:
            ag.add_edge(edge[0], edge[1])
        q = deque()
        for (i, indegree) in enumerate(ag.indegrees):
            if indegree == 0:
                q.append(i)
        ans = []
        while q:
            mm = len(q)
            for _ in range(mm):
                node = q.popleft()
                ans.append(node)
                for neighbor in ag.adj_graph[node]:
                    ag.indegrees[neighbor] -= 1
                    if ag.indegrees[neighbor] == 0:
                        q.append(neighbor)
        if len(ans) != numCourses:
            return []
        return ans[::-1]
        


