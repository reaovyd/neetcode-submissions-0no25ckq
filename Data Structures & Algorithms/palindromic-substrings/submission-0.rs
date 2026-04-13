impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let n = s.len() as i32;
        let mut res = 0;
        (0..n).for_each(|i| {
            let mut i = i;
            let k = i;
            let mut j = i;
            while i >= 0 && j < n && s[i as usize] == s[j as usize] {
                res += 1;
                i -= 1;
                j += 1;
            }
            let mut i = k;
            let mut j = i + 1;
            while i >= 0 && j < n && s[i as usize] == s[j as usize] {
                res += 1;
                i -= 1;
                j += 1;
            }
        });

        res
    }
}
