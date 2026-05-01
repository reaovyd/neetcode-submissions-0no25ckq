impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut res = 0;
        let is_neg = x < 0;
        while x != 0 {
            let mut digit = x % 10;
            if is_neg {
                digit = -digit;
            }
            if res > (i32::MAX - digit) / 10 {
                return 0;
            }
            if res < (i32::MIN + digit) / 10 {
                return 0;
            }
            res *= 10;
            res += digit;
            x /= 10;
        }
        if is_neg {
            res *= -1;
        }
        res
    }
}