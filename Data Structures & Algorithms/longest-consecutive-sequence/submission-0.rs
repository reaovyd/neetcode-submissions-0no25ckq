use std::{cmp::max, collections::HashSet};

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set = nums.into_iter().collect::<HashSet<i32>>();
        let mut ans = 0;
        for num in set.iter() {
            if !set.contains(&(*num - 1)) {
                let mut val = *num;
                while set.contains(&val) {
                    val += 1;
                }
                ans = max(ans, val - *num);
            }
        }
        ans
    }
}
