impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let n = digits.len();
        digits[n - 1] += 1;
        for i in (1..n).rev() {
            if digits[i] >= 10 {
                let num = digits[i] % 10;
                let num2 = (digits[i] / 10) % 10;
                digits[i] = num;
                digits[i - 1] += num2;
            }
        }

        if digits[0] >= 10 {
            let num = digits[0] % 10;
            let num2 = (digits[0] / 10) % 10;
            digits.insert(0, 0);
            digits[1] = num;
            digits[0] += num2;
        }

        digits
    }
}
