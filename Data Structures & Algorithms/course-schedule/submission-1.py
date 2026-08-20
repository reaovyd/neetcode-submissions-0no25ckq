class AdjGraph:
    node_edges: List[Set[int]]

    def __init__(self, num_courses: int):
        self.node_edges = [set() for i in range(num_courses)]
    
    def add_edge(self, a: int, b: int):
        self.node_edges[a].add(b)

class Solution:
    def canFinish(self, numCourses: int, prerequisites: List[List[int]]) -> bool:
        indegrees = [0 for _ in range(numCourses)]
        graph = AdjGraph(numCourses)

        for edge in prerequisites:
            indegrees[edge[1]] += 1
            graph.add_edge(edge[0], edge[1])
        q = deque()
        for (i, indegree) in enumerate(indegrees):
            if indegree == 0:
                q.append(i)

        co = 0
        while q:
            nn = len(q)
            for _ in range(nn):
                qi = q.popleft()
                co += 1
                for node in graph.node_edges[qi]:
                    indegrees[node] -= 1
                    if indegrees[node] == 0:
                        q.append(node)
        return co == numCourses