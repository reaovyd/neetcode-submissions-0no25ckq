class UnionFind:
    groups: List[int]
    groups_len: int
    sizes: List[int]

    def __init__(self, n: int):
        self.groups = [i for i in range(n)]
        self.groups_len = n
        self.sizes = [1 for _ in range(n)]

    def find(self, a: int) -> int:
        tt = a
        p = self.groups[tt]
        while p != tt:
            tt = p
            p = self.groups[p]
        ancestor = p
        p = self.groups[a]
        while p != ancestor:
            pp = self.groups[p]
            self.groups[a] = ancestor
            a = p
            p = pp
        return ancestor
    def union(self, a: int, b: int) -> bool:
        p_a = self.find(a)
        p_b = self.find(b)
        if p_a == p_b:
            return False
        else:
            if self.sizes[p_a] < self.sizes[p_b]:
                self.groups[p_a] = p_b
                self.sizes[p_b] += self.sizes[p_a]
            else:
                self.groups[p_b] = p_a
                self.sizes[p_a] += self.sizes[p_b]
        self.groups_len -= 1
        return True

class Solution:
    def minCostConnectPoints(self, points: List[List[int]]) -> int:
        n = len(points)
        edges = []
        for i in range(n):
            for j in range(i + 1, n):
                dx = abs(points[i][0] - points[j][0])
                dy = abs(points[i][1] - points[j][1])
                edges.append((i, j, dx + dy))
        edges = sorted(edges, key = lambda edge: edge[2])
        uf = UnionFind(n)
        ans = 0
        for edge in edges:
            cond = uf.union(edge[0], edge[1])
            if cond:
                ans += edge[2]
            if uf.groups_len == 1:
                break
        return ans