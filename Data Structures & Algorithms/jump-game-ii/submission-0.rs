use std::cmp::max;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, 0);
        let mut res = 0;
        let n = nums.len();
        while r < n - 1 {
            let mut largest_jump = 0;
            for i in l..=r {
                largest_jump = max(nums[i] + i as i32, largest_jump);
            }
            l = r + 1;
            r = largest_jump as usize;
            res += 1
        }
        res
    }
}
