impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut ss = [0; 26];
        let mut tt = [0; 26];
        s.chars().for_each(|c| {
            ss[(c as u8 - b'a') as usize] += 1;
        });
        t.chars().for_each(|c| {
            tt[(c as u8 - b'a') as usize] += 1;
        });
        ss == tt
    }
}
