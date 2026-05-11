use std::{cell::RefCell, collections::HashSet, rc::Rc};

type TrieLink = Option<Rc<RefCell<TrieNode>>>;

struct TrieNode {
    chars: Vec<TrieLink>,
    is_word: bool,
}

impl TrieNode {
    pub fn new() -> Self {
        Self { chars: vec![None; 26], is_word: false }
    }
}

struct Trie {
    root: TrieLink,
}

impl Trie {
    pub fn new() -> Self {
        Self { root: Some(Rc::new(RefCell::new(TrieNode::new()))) }
    }

    pub fn add_word(&self, word: &str) {
        let mut root_link = self.root.clone();
        for c in word.chars() {
            let c = c as u8 - b'a';
            root_link = root_link.map(|node| {
                let mut brw = node.borrow_mut();
                if let Some(ref new_node) = brw.chars[c as usize] {
                    new_node.clone()
                } else {
                    let rf = Rc::new(RefCell::new(TrieNode::new()));
                    brw.chars[c as usize].replace(rf.clone());
                    rf
                }
            });
        }
        root_link.unwrap().borrow_mut().is_word = true;
    }

    pub fn search_word(&self, word: &str) -> bool {
        let mut root_link = self.root.clone();
        for c in word.chars() {
            let c = c as u8 - b'a';
            if let Some(Some(new_root)) = root_link.map(|node| {
                let brw = node.borrow_mut();
                brw.chars[c as usize].as_ref().map(|new_node| new_node.clone())
            }) {
                root_link = Some(new_root);
            } else {
                return false;
            }
        }
        root_link.unwrap().borrow().is_word
    }
}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let trie = Trie::new();
        let n = board.len();
        let m = board[0].len();
        let mut res = HashSet::new();
        for word in words {
            trie.add_word(&word);
        }
        for i in 0..n {
            for j in 0..m {
                let mut visited = vec![vec![false; m]; n];
                let mut str_stack = String::new();
                dfs(&board, &trie.root, &mut visited, &mut str_stack, &mut res, i as i32, j as i32);
            }
        }
        res.into_iter().collect()
    }
}

fn dfs(
    board: &[Vec<char>],
    trie_link: &TrieLink,
    visited: &mut [Vec<bool>],
    str_stack: &mut String,
    res: &mut HashSet<String>,
    i: i32,
    j: i32,
) {
    if i >= board.len() as i32
        || i < 0
        || j >= board[0].len() as i32
        || j < 0
        || visited[i as usize][j as usize]
    {
        return;
    }
    if let Some(link) = trie_link {
        let current_char = board[i as usize][j as usize] as u8 - b'a';
        str_stack.push((current_char + b'a') as char);
        let next_node = link.borrow().chars[current_char as usize].clone();
        if let Some(ref next_node) = next_node {
            if next_node.borrow().is_word {
                res.insert(str_stack.clone());
            }
        }
        visited[i as usize][j as usize] = true;
        dfs(board, &next_node, visited, str_stack, res, i + 1, j);
        dfs(board, &next_node, visited, str_stack, res, i - 1, j);
        dfs(board, &next_node, visited, str_stack, res, i, j + 1);
        dfs(board, &next_node, visited, str_stack, res, i, j - 1);
        visited[i as usize][j as usize] = false;
        str_stack.pop();
    }
}
