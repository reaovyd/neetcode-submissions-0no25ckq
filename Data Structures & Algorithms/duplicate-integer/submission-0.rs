use std::collections::HashSet;

impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        nums.iter().collect::<HashSet<_>>().len() != nums.len()
    }
}
