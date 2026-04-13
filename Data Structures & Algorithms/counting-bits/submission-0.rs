impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ans = Vec::new();
        for i in (0..n + 1) {
            let mut i = i;
            let mut inp = 0;
            while i > 0 {
                inp += i & 1;
                i >>= 1;
            }
            ans.push(inp);
        }
        ans
    }
}
