use std::cmp::min;


impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut i = 1;
        let mut j = u32::MAX - 1;
        let mut ans = u32::MAX;
        while i <= j {
            let m = (i + j) >> 1;
            let mut res = 0;
            for pile in piles.iter() {
                let pile = *pile as u32;
                if pile.is_multiple_of(m) {
                    res += pile / m;
                } else {
                    res += (pile / m) + 1;
                }
            }
            if res <= h as u32 {
                ans = min(ans, m);
                j = m - 1;
            } else {
                i = m + 1;
            }
        }
        ans as i32
    }
}
