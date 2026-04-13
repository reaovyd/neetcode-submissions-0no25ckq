use std::{
    cmp::{Reverse, min},
    collections::BinaryHeap,
};

struct Node {
    dist: i32,
    node_id: i32,
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist.cmp(&other.dist)
    }
}

struct Graph {
    lst: Vec<Vec<(i32, i32)>>,
}

impl Graph {
    pub fn new(n: i32) -> Self {
        let n = n as usize;
        let lst = vec![vec![]; n + 1];
        Self { lst }
    }

    pub fn add(&mut self, node1: i32, node2: i32, edge_weight: i32) {
        self.lst[node1 as usize].push((node2, edge_weight));
    }
}

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let mut dist = vec![i32::MAX; (n as usize) + 1];
        let mut graph = Graph::new(n);
        let mut visited = vec![false; (n as usize) + 1];

        for time in times {
            graph.add(time[0], time[1], time[2]);
        }
        dist[k as usize] = 0;
        let mut heap = BinaryHeap::new();
        heap.push(Reverse(Node {
            node_id: k,
            dist: dist[k as usize],
        }));

        while let Some(node) = heap.pop() {
            let node = node.0;
            if visited[node.node_id as usize] {
                continue;
            }
            visited[node.node_id as usize] = true;
            let adjs = &graph.lst[node.node_id as usize];
            for adj in adjs {
                dist[adj.0 as usize] =
                    min(dist[adj.0 as usize], dist[node.node_id as usize] + adj.1);
                heap.push(Reverse(Node {
                    node_id: adj.0,
                    dist: dist[adj.0 as usize],
                }));
            }
        }
        let ans = *dist[1..].iter().max().unwrap();
        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}
