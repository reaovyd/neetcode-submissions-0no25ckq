use std::{cell::RefCell, rc::Rc};

type TrieLink = Option<Rc<RefCell<TrieNode>>>;

struct TrieNode {
    alph: Vec<TrieLink>,
    is_word: bool,
}

impl TrieNode {
    pub fn new() -> Self {
        Self { alph: vec![None; 26], is_word: false }
    }
}

struct WordDictionary {
    root: TrieLink,
}

impl WordDictionary {
    fn new() -> Self {
        Self { root: Some(Rc::new(RefCell::new(TrieNode::new()))) }
    }

    fn add_word(&self, word: String) {
        let mut root_link = self.root.clone();
        for c in word.chars() {
            let c = c as u8 - b'a';
            root_link = {
                // this is safe to unwrap since we always set it to Some() value
                let root_link = root_link.take().unwrap();
                let mut brw = root_link.borrow_mut();
                if let Some(ref node) = brw.alph[c as usize] {
                    Some(node.clone())
                } else {
                    brw.alph[c as usize] = Some(Rc::new(RefCell::new(TrieNode::new())));
                    brw.alph[c as usize].clone()
                }
            };
        }
        root_link.as_ref().unwrap().borrow_mut().is_word = true;
    }

    fn search(&self, word: String) -> bool {
        let root_link = self.root.clone();
        self._search(&word.chars().collect::<Vec<char>>(), 0, root_link)
    }

    fn _search(&self, word: &[char], i: usize, link: TrieLink) -> bool {
        if i >= word.len() {
            link.as_ref().is_some_and(|node| node.borrow().is_word)
        } else {
            let c = word[i];
            if let Some(link) = link {
                let mut cond = false;
                if c == '.' {
                    for cc in link.borrow().alph.iter() {
                        cond = self._search(word, i + 1, cc.clone());
                        if cond {
                            break;
                        }
                    }
                    cond
                } else {
                    let c = c as u8 - b'a';
                    self._search(word, i + 1, link.borrow().alph[c as usize].clone())
                }
            } else {
                false
            }
        }
    }
}
