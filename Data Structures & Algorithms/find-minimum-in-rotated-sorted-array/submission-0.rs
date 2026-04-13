impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let (mut i, mut j) = (0, n - 1);
        while i < j {
            let m = (j + i) >> 1;
            if nums[m] > nums[j] {
                i = m + 1;
            } else {
                j = m;
            }
        }
        nums[i]
    }
}
