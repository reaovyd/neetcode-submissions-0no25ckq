use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut queue = VecDeque::new();
        let word_list = word_list.into_iter().collect::<HashSet<String>>();
        let mut visited = HashSet::new();
        visited.insert(begin_word.clone());
        let mut res = 1;
        queue.push_back(begin_word);
        while !queue.is_empty() {
            let mut m = queue.len();
            while m > 0 {
                let word = queue.pop_front().unwrap();
                if word == end_word {
                    return res;
                }
                let vec = word.chars().collect::<Vec<char>>();
                let n = vec.len();
                for i in 0..n {
                    for cc in 'a'..='z' {
                        if cc == vec[i] {
                            continue;
                        }
                        let mut candidate = vec.iter().collect::<String>();
                        candidate.replace_range(
                            word.char_indices()
                                .nth(i)
                                .map(|(pos, ch)| pos..pos + ch.len_utf8())
                                .unwrap(),
                            &cc.to_string(),
                        );
                        if word_list.contains(&candidate) && !visited.contains(&candidate) {
                            visited.insert(candidate.clone());
                            queue.push_back(candidate);
                        }
                    }
                }
                m -= 1;
            }
            res += 1;
        }
        0
    }
}
