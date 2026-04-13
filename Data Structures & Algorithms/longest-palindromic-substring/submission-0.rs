use std::collections::VecDeque;


impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s = s.chars().collect::<Vec<char>>();
        let n = s.len() as i32;
        let mut res = String::new();
        (0..n).for_each(|i| {
            let mut i = i;
            let k = i;
            let mut j = i;
            let mut p = VecDeque::new();
            while i >= 0 && j < n && s[i as usize] == s[j as usize] {
                if i == j {
                    p.push_front(s[i as usize]);
                } else {
                    p.push_front(s[i as usize]);
                    p.push_back(s[j as usize]);
                }
                i -= 1;
                j += 1;
            }
            if p.len() > res.len() {
                res = p.into_iter().collect::<String>();
            }
            let mut p = VecDeque::new();
            let mut i = k;
            let mut j = i + 1;
            while i >= 0 && j < n && s[i as usize] == s[j as usize] {
                p.push_front(s[i as usize]);
                p.push_back(s[j as usize]);
                i -= 1;
                j += 1;
            }
            if p.len() > res.len() {
                res = p.into_iter().collect::<String>();
            }
        });

        res
    }
}
