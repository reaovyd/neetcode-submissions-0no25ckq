impl Solution {
    pub fn is_palindrome(mut s: String) -> bool {
        let s = s.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect::<Vec<_>>();
        let n = s.len();
        let mut i = 0 as i32;
        let mut j = n as i32 - 1;
        while i < j {
            if s[i as usize] != s[j as usize] {
                return false;
            } else {
                i += 1;
                j -= 1;
            }
        }
        true
    }
}
