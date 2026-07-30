impl Solution {
    pub fn find_duplicate(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        // [1,3,4,2,2]
        for i in 0..n {
            while i != (nums[i] - 1) as usize {
                if nums[nums[i] as usize - 1] != nums[i] {
                    let ni = nums[i];
                    nums.swap(i, ni as usize - 1);
                } else {
                    return nums[i];
                }
            }
        }
        for i in 1..n {
            if nums[i - 1] + 1 != nums[i] {
                return nums[i];
            }
        }
        -1
    }
}
