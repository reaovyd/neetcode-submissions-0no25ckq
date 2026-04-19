use std::cmp::max;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = height.len() - 1;
        let mut ans = 0;
        while i < j {
            let width = (j - i) as i32;
            if height[i] < height[j] {
                ans = max(ans, height[i] * width);
                i += 1;
            } else {
                ans = max(ans, height[j] * width);
                j -= 1;
            }
        }
        ans
    }
}
