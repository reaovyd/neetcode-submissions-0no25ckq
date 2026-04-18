use std::collections::VecDeque;

struct Graph {
    lst: Vec<Vec<i32>>,
    indegrees: Vec<i32>,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        Graph { lst: vec![vec![]; n], indegrees: vec![0; n] }
    }

    pub fn add_edge(&mut self, a: i32, b: i32) {
        self.lst[a as usize].push(b);
        self.indegrees[b as usize] += 1;
    }
}

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut g = Graph::new(num_courses as usize);
        for edge in prerequisites {
            g.add_edge(edge[0], edge[1]);
        }
        let mut queue = VecDeque::new();
        let mut vec = vec![];
        g.indegrees.iter().enumerate().filter(|(_, num)| **num == 0).map(|(idx, _)| idx).for_each(
            |idx| {
                queue.push_front(idx);
            },
        );
        while !queue.is_empty() {
            let sz = queue.len();
            for _ in 0..sz {
                let idx = queue.pop_back().unwrap();
                vec.push(idx as i32);

                for num in g.lst[idx].iter() {
                    g.indegrees[*num as usize] -= 1;
                    if g.indegrees[*num as usize] == 0 {
                        queue.push_front(*num as usize);
                    }
                }
            }
        }
        if g.indegrees.into_iter().all(|num| num == 0) {
            vec.reverse();
            vec
        } else {
            vec![]
        }
    }
}
