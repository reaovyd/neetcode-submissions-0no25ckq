use std::cell::RefCell;
use std::rc::Rc;
use std::collections::{HashMap, VecDeque};

/*
// Definition for a Node.
#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub val: i32,
    pub neighbors: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    #[inline]
    pub fn new(val: i32) -> Self {
        Node {
            val,
            neighbors: Vec::new(),
        }
    }
}
*/


impl Solution {
    pub fn clone_graph(node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        let mut queue = VecDeque::new();
        let mut map = HashMap::new();
        queue.push_back(node?);
        while let Some(node) = queue.pop_front() {
            let node = node.borrow();
            map.insert(
                node.val,
                (
                    Rc::new(RefCell::new(Node::new(node.val))),
                    node.neighbors
                        .iter()
                        .map(|new_node| new_node.borrow().val)
                        .collect::<Vec<i32>>(),
                ),
            );
            for neighbor in node.neighbors.iter() {
                let cond = {
                    let borrow = neighbor.borrow();
                    map.contains_key(&borrow.val)
                };
                if !cond {
                    queue.push_back(neighbor.clone());
                }
            }
        }
        for node in map.values() {
            node.0.borrow_mut().neighbors =
                node.1.iter().map(|num| map.get(num).unwrap().0.clone()).collect::<Vec<_>>();
        }
        Some(map.get(&1).unwrap().0.clone())
    }
}

