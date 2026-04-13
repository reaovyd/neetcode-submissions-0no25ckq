impl Solution {
    pub fn my_pow(x: f64, mut n: i32) -> f64 {
        if n < 0 {
            let mut num = x;
            if n == i32::MIN {
                num = 1.0;
                n += 1;
            }
            n = -n;
            1.0 / (Solution::_my_pow(x, n) * (x / num))
        } else {
            Solution::_my_pow(x, n)
        }
    }
    pub fn _my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            1.0
        } else if n % 2 == 1 {
            x * Solution::_my_pow(x, (n - 1) / 2) * Solution::_my_pow(x, (n - 1) / 2)
        } else {
            Solution::_my_pow(x, n / 2) * Solution::_my_pow(x, n / 2)
        }
    }
}
