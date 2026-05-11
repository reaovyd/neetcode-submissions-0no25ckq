use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap},
    rc::Rc,
};

#[derive(Debug, Clone)]
struct List {
    head: Link,
    tail: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: None, tail: None }
    }

    pub fn pop_front(&mut self) -> Link {
        self.head.take().inspect(|head_node| {
            if let Some(next_node) = head_node.borrow_mut().next.take() {
                next_node.borrow_mut().prev.take();
                self.head = Some(next_node);
            }
            if self.head.is_none() {
                self.tail = None;
            }
        })
    }

    pub fn push_back_link(&mut self, node: Link) {
        if let Some(tail_node) = self.tail.take() {
            tail_node.borrow_mut().next = node.clone();
            node.clone().unwrap().borrow_mut().prev = Some(tail_node);
            self.tail = node;
        } else {
            self.tail = node.clone();
            self.head = node;
        }
    }

    pub fn pop_node(&mut self, node: Link) -> Link {
        let node = node?;
        match (&node.borrow().prev, &node.borrow().next) {
            (None, None) => {
                self.head = None;
                self.tail = None;
            }
            (None, Some(next)) => {
                self.head = Some(next.clone());
                next.borrow_mut().prev.take();
            }
            (Some(prev), None) => {
                self.tail = Some(prev.clone());
                prev.borrow_mut().next.take();
            }
            (Some(prev), Some(next)) => {
                prev.borrow_mut().next = Some(next.clone());
                next.borrow_mut().prev = Some(prev.clone());
            }
        };
        node.borrow_mut().next = None;
        node.borrow_mut().prev = None;
        Some(node)
    }
}

impl Drop for List {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
struct Node {
    value: i32,
    prev: Link,
    next: Link,
}

impl Node {
    pub fn new(value: i32, prev: Link, next: Link) -> Self {
        Self { value, prev, next }
    }
}

struct LFUCache {
    min_freq: usize,
    freq_map: HashMap<usize, List>,
    map: HashMap<i32, (i32, usize, Link)>,
    capacity: usize,
}

impl LFUCache {
    fn new(capacity: i32) -> Self {
        Self {
            min_freq: 1,
            freq_map: HashMap::new(),
            map: HashMap::new(),
            capacity: capacity as usize,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Entry::Occupied(occupied_entry) = self.map.entry(key) {
            let entry = occupied_entry.into_mut();
            let node = self.freq_map.get_mut(&entry.1).unwrap().pop_node(entry.2.clone());
            if entry.1 == self.min_freq && self.freq_map[&entry.1].head.is_none() {
                self.min_freq += 1;
            }
            entry.1 += 1;
            self.freq_map.entry(entry.1).or_insert(List::new()).push_back_link(node);
            entry.0
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Entry::Occupied(occupied_entry) = self.map.entry(key) {
            let entry = occupied_entry.into_mut();
            let node = self.freq_map.get_mut(&entry.1).unwrap().pop_node(entry.2.clone());
            if entry.1 == self.min_freq && self.freq_map[&entry.1].head.is_none() {
                self.min_freq += 1;
            }
            entry.1 += 1;
            self.freq_map.entry(entry.1).or_insert(List::new()).push_back_link(node);
            entry.0 = value;
        } else {
            if self.map.len() == self.capacity {
                if let Some(node) = self.freq_map.get_mut(&self.min_freq).unwrap().pop_front() {
                    self.map.remove(&node.borrow().value);
                }
            }
            let node = Rc::new(RefCell::new(Node::new(key, None, None)));
            self.min_freq = 1;
            self.map.insert(key, (value, self.min_freq, Some(node.clone())));
            self.freq_map.entry(self.min_freq).or_insert(List::new()).push_back_link(Some(node));
        }
    }
}
