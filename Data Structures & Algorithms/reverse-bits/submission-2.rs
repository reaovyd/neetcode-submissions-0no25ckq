impl Solution {
    pub fn reverse_bits(mut n: u32) -> u32 {
        let mut i = 31;
        let mut ans = 0;
        while i > 0 {
            let b = n & 1;
            ans |= b;
            ans <<= 1;
            n >>= 1;
            i -= 1;
        }
        ans | n
    }


}
