impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut min_idx = n - 1;
        for i in (0..n).rev() {
            if i as i32 + nums[i] >= min_idx as i32 {
                min_idx = i;
            }
        }
        min_idx == 0 
    }
}