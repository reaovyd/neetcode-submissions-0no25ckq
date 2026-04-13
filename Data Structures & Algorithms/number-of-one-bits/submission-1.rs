impl Solution {
    pub fn hamming_weight(mut n: u32) -> u32 {
        let mut ans = 0;
        while n > 0 {
            ans += n & 1;
            n >>= 1;
        }
        ans
    }

}
