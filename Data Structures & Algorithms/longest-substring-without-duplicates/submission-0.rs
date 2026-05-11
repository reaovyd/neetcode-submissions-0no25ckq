use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut alph = HashSet::new();
        let n = chars.len();
        let mut i = 0;
        let mut j = 0;
        let mut ans = 0;
        while j < n {
            let cj = chars[j] as u8 - b'a';
            if alph.contains(&cj) {
                let ci = chars[i] as u8 - b'a';
                alph.remove(&ci);
                i += 1;
            } else {
                alph.insert(cj);
                ans = ans.max(j - i + 1);
                j += 1;
            }
        }
        ans as i32
    }
}
