use std::collections::HashSet;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut n = n.to_string();
        let mut set = HashSet::new();
        loop {
            let prev_n = n.clone();
            set.insert(prev_n);
            let mut sum = 0;
            for c in n.chars() {
                let num = (c as u8 - b'0') as u32;
                sum += num * num;
            }
            if sum == 1 {
                return true;
            } else {
                n = sum.to_string();
                if set.contains(&n) {
                    break;
                }
            }
        }
        false
    }
}
