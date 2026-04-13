/**
 * Definition of Interval:
 * #[derive(Debug, Clone)]
 * pub struct Interval {
 *     pub start: i32,
 *     pub end: i32,
 * }
 *
 * impl Interval {
 *     pub fn new(start: i32, end: i32) -> Self {
 *         Interval { start, end }
 *     }
 * }
 */

use std::{
    cmp::{max, Ordering, Reverse},
    collections::BinaryHeap,
};

impl Solution {
    pub fn min_meeting_rooms(mut intervals: Vec<Interval>) -> i32 {
        intervals.sort_by(|i1, i2| {
            if i1.start > i2.start {
                Ordering::Greater
            } else if i1.start < i2.start {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });
        if intervals.is_empty() {
            return 0;
        }

        let mut heap = BinaryHeap::new();
        heap.push(Reverse(intervals[0].end));
        let n = intervals.len();
        (1..n).for_each(|i| {
            if intervals[i].start >= heap.peek().unwrap().0 {
                heap.pop();
            }
            heap.push(Reverse(intervals[i].end));
        });
        heap.len() as i32
    }
}
