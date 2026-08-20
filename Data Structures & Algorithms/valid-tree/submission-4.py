class UnionFind:
    union_groups: List[int]
    sizes: List[int]

    def __init__(self, n: int):
        self.union_groups = [i for i in range(n)]
        self.sizes = [1 for i in range(n)]
    
    def find(self, a: int) -> int:
        t = a
        p = self.union_groups[t]
        while p != t:
            t = p
            p = self.union_groups[t]
        ancestor = p

        p = self.union_groups[a]
        while a != p:
            self.union_groups[a] = ancestor
            a = p
            p = self.union_groups[p]
        return p
    
    def union(self, a: int, b: int):
        pp_a = self.find(a)
        pp_b = self.find(b)
        if pp_a == pp_b:
            return
        if self.sizes[pp_a] < self.sizes[pp_b]:
            self.union_groups[pp_a] = pp_b
            self.sizes[pp_b] += self.sizes[pp_a]
        else:
            self.union_groups[pp_b] = pp_a
            self.sizes[pp_a] += self.sizes[pp_b]

          

class Solution:
    def validTree(self, n: int, edges: List[List[int]]) -> bool:
        if len(edges) != n - 1:
            return False
        uf = UnionFind(n)
        
        for edge in edges:
            if uf.union_groups[edge[0]] != uf.union_groups[edge[1]]:
                uf.union(edge[0], edge[1])
            else:
                return False
        s = set()
        for node in range(n):
            s.add(uf.find(node))
        if len(s) != 1:
            return False        
        return True        
