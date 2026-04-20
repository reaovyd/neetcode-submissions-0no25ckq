use std::cmp::{max, min};

impl Solution {
    pub fn trap(mut height: Vec<i32>) -> i32 {
        let mut lft = vec![];
        let mut rht = vec![];
        let n = height.len();
        let mut mx = 0;
        for i in 0..n {
            lft.push(mx);
            mx = max(mx, height[i]);
        }
        mx = 0;
        for i in (0..n).rev() {
            rht.push(mx);
            mx = max(mx, height[i]);
        }
        rht.reverse();
        let mut ans = 0;
        for i in 0..n {
            let h = min(lft[i], rht[i]) - height[i];
            if h > 0 {
                ans += h;
            }
        }
        ans
    }
}
