class UnionFind:
    groups: List[Set[int]]
    sizes: List[int]
    def __init__(self, n: int):
        self.groups = [i for i in range(n + 1)]
        self.sizes = [1 for _ in range(n + 1)]
    
    def find(self, a: int) -> int:
        tt = a
        p = self.groups[a]
        while p != tt:
            tt = p
            p = self.groups[p]
        
        ancestor = p
        p = self.groups[a]
        while ancestor != a:
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
        return True

class Solution:
    def findRedundantConnection(self, edges: List[List[int]]) -> List[int]:
        uf = UnionFind(len(edges))
        for edge in edges:
            if not uf.union(edge[0], edge[1]):
                return edge
        return []