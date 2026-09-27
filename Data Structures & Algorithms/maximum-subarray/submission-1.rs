use std::cmp::max;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut ans = i32::MIN;
        let mut cur_sum = 0;
        for num in nums {
            cur_sum += num;
            ans = max(ans, cur_sum);
            if cur_sum < 0 {
                cur_sum = 0;
            }
        }
        ans
    }
}