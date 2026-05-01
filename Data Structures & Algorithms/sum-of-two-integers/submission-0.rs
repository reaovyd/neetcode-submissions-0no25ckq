impl Solution {
    pub fn get_sum(mut a: i32, mut b: i32) -> i32 {
        let mut carry = 0;
        let mut res = 0;
        for i in 0..32 {
            let aa = (a & 1);
            let bb = (b & 1);
            let mut or = aa | bb;
            if aa == 1 && bb == 1 {
                if carry == 1 {
                    or |= carry;
                } else {
                    or = 0;
                }
                carry |= 1;
            } else if aa == 0 && bb == 1 {
                if carry == 1 {
                    or |= carry;
                    carry |= 1;
                    or = 0;
                } else {
                    or = 1;
                }
            } else if aa == 1 && bb == 0 {
                if carry == 1 {
                    or |= carry;
                    carry |= 1;
                    or = 0;
                } else {
                    or = 1;
                }
            } else {
                if carry == 1 {
                    or |= carry;
                }
                carry = 0;
            }
            res |= (or << i);
            a >>= 1;
            b >>= 1;
        }
        res
    }
}