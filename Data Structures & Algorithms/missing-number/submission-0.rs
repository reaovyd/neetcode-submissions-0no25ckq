impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let a1 = (0..=nums.len() as i32).fold(0, |a, e| a ^ e);
        nums.into_iter().fold(a1, |a, e| a ^ e)
    }
}
