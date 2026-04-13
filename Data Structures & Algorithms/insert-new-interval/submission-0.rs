use std::cmp::{max, Ordering};


impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            intervals.push(new_interval);
            intervals
        } else {
            let idx = search(&intervals, &new_interval);
            intervals.insert(idx, new_interval);
            Solution::merge(intervals)
        }
    }
}
pub fn search(intervals: &[Vec<i32>], new_interval: &[i32]) -> usize {
    let mut i = 0;
    let mut j = intervals.len();
    while i < j {
        if new_interval[0] >= intervals[i][0] {
            return i;
        }
        i += 1;
    }
    return j;
}

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by(|i1, i2| {
            if i1[0] > i2[0] {
                Ordering::Greater
            } else if i1[0] < i2[0] {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });
        let mut st = Vec::with_capacity(intervals.len());
        let mut iter = intervals.into_iter();
        st.push(iter.next().unwrap());
        for itv in iter {
            let last_elem = st.last().unwrap();
            if last_elem[1] >= itv[0] {
                let new = vec![last_elem[0], max(last_elem[1], itv[1])];
                st.pop();
                st.push(new);
            } else {
                st.push(itv);
            }
        }
        st
    }
}
