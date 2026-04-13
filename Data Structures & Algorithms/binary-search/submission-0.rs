impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut i, mut j) = (0, nums.len() - 1);
        while i <= j {
            let m = (i + j) >> 1;
            if nums[m] == target {
                return m as i32;
            } else if nums[m] > target {
                if m == 0 {
                    return -1;
                }
                j = m - 1;
            } else {
                if m == nums.len() - 1 {
                    return -1;
                }
                i = m + 1;
            }
        }
        -1
    }
}
