class UnionFind:
    groups: List[int]
    sizes: List[int]

    def __init__(self, n: int):
        self.groups = [i for i in range(n)]
        self.sizes = [1 for _ in range(n)]
    
    def find(self, a: int) -> int:
        tt = a
        p = self.groups[tt]
        while p != tt:
            tt = p
            p = self.groups[p]

        ancestor = p
        p = self.groups[a]

        # path minimizing
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
        return True
    def count_connected_components(self) -> int:
        components = set()
        for i in range(len(self.groups)):
            p = self.find(i)
            components.add(p)
        return len(components)

    
class Solution:
    def validTree(self, n: int, edges: List[List[int]]) -> bool:
        uf = UnionFind(n)
        for edge in edges:
            if not uf.union(edge[0], edge[1]):
                return False
        return uf.count_connected_components() == 1
        