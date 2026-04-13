use std::cmp::{Ordering};
impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_by(|i1, i2| {
            if i1[0] > i2[0] {
                Ordering::Greater
            } else if i1[0] < i2[0] {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });
        let n = intervals.len();
        let mut res = vec![&intervals[0]];
        (1..n).for_each(|i| {
            if intervals[i][1] < res.last().unwrap()[1] {
                res.pop();
                res.push(&intervals[i]);
            } else if intervals[i][0] >= res.last().unwrap()[1] {
                res.push(&intervals[i]);
            }
        });

        (n - res.len()) as i32
    }
}
