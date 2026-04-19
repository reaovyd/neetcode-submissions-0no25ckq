impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut prefix = vec![1; n];
        for i in 1..n {
            prefix[i] = prefix[i - 1] * nums[i - 1]
        }
        let mut postfix = 1;
        for i in (0..n).rev() {
            prefix[i] *= postfix;
            postfix *= nums[i];
        }
        prefix
    }
}
