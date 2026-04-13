struct Graph {
    adj_list: Vec<Vec<i32>>,
}

impl Graph {
    pub fn new(n: i32) -> Self {
        Graph { adj_list: vec![vec![]; n as usize] }
    }

    pub fn add_edge(&mut self, u: i32, v: i32) {
        self.adj_list[u as usize].push(v);
        self.adj_list[v as usize].push(u);
    }
}

impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        if edges.len() as i32 != n - 1 {
            return false;
        }
        let mut graph = Graph::new(n);
        let mut visited = vec![false; n as usize];
        for edge in edges {
            graph.add_edge(edge[0], edge[1]);
        }
        Self::dfs(&graph, &mut visited, 0, -1) && visited.iter().all(|elem| *elem)
    }

    pub fn dfs(graph: &Graph, visited: &mut [bool], node: i32, parent: i32) -> bool {
        if visited[node as usize] {
            return false;
        }

        visited[node as usize] = true;
        for node_y in graph.adj_list[node as usize].iter() {
            if *node_y != parent && !Self::dfs(graph, visited, *node_y, node) {
                return false;
            }
        }
        true
    }
}
