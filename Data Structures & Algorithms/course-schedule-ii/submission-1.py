class AdjGraph:
    graph: List[Set[int]]

    def __init__(self, numCourses: int):
        self.graph = [set() for _ in range(numCourses)]
    
    def add_edge(self, a: int, b: int):
        self.graph[a].add(b)

class Solution:
    def findOrder(self, numCourses: int, prerequisites: List[List[int]]) -> List[int]:
        graph = AdjGraph(numCourses)
        indegrees = [0 for _ in range(numCourses)]
        
        for edge in prerequisites:
            indegrees[edge[1]] += 1
            graph.add_edge(edge[0], edge[1])
        
        q = deque()
        for (i, indegree) in enumerate(indegrees):
            if indegree == 0:
                q.append(i)
        
        ans = []
        while q:
            mm = len(q)
            for i in range(mm):
                a = q.popleft()
                ans.append(a)
                for node in graph.graph[a]:
                    indegrees[node] -= 1
                    if indegrees[node] == 0:
                        q.append(node)
        if len(ans) != numCourses:
            return []
        return ans[::-1]