use std::collections::{BinaryHeap, HashMap};

struct Node {
    key: i32,
    val: i32,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.val.cmp(&other.val)
    }
}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut heap = BinaryHeap::new();
        let mut map = HashMap::new();
        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }
        for (key, val) in map {
            let node = Node { key, val };
            heap.push(node);
        }

        let mut vec = vec![];
        for _ in 0..k {
            vec.push(heap.pop().unwrap().key);
        }
        vec
    }
}
