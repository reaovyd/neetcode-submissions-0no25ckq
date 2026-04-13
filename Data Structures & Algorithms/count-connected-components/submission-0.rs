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
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut graph = Graph::new(n);
        let mut visited = vec![false; n as usize];
        for edge in edges {
            graph.add_edge(edge[0], edge[1]);
        }
        let mut ans = 0;
        for i in (0..n) {
            if visited[i as usize] {
                continue;
            }
            Self::dfs(&graph, &mut visited, i);
            ans += 1;
        }
        ans
    }

    fn dfs(graph: &Graph, visited: &mut [bool], node: i32) {
        if visited[node as usize] {
            return;
        }
        visited[node as usize] = true;
        for node_y in graph.adj_list[node as usize].iter() {
            Self::dfs(graph, visited, *node_y);
        }
    }
}
