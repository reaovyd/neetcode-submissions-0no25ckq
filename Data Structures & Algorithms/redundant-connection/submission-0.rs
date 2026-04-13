struct UnionFind {
    parent: Vec<i32>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self { parent: (0..(n + 1)).map(|num| num as i32).collect(), size: vec![1; n + 1] }
    }

    pub fn find(&mut self, mut node: i32) -> i32 {
        while self.parent[node as usize] != node {
            self.parent[node as usize] = self.parent[self.parent[node as usize] as usize];
            node = self.parent[node as usize];
        }
        node
    }

    pub fn size(&self, node: i32) -> usize {
        self.size[node as usize]
    }

    pub fn union(&mut self, node1: i32, node2: i32) -> bool {
        let p1 = self.find(node1);
        let p2 = self.find(node2);
        if p1 == p2 {
            return false;
        }
        if self.size(p1) < self.size(p2) {
            self.parent[p1 as usize] = p2;
            self.size[p2 as usize] += self.size[p1 as usize];
        } else {
            self.parent[p2 as usize] = p1;
            self.size[p1 as usize] += self.size[p2 as usize];
        }
        true
    }
}

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut uf = UnionFind::new(n);
        for edge in edges {
            let (a, b) = (edge[0], edge[1]);
            if !uf.union(a, b) {
                return edge;
            }
        }
        unimplemented!()
    }
}
