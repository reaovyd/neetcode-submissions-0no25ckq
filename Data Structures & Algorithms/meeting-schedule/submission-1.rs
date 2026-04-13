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

impl Solution {
    pub fn can_attend_meetings(mut intervals: Vec<Interval>) -> bool {
        let n = intervals.len();
        intervals.sort_by(|interval1, interval2| {
            if interval1.start > interval2.start {
                Ordering::Greater
            } else if interval1.start < interval2.start {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });

        for i in 1..n {
            if intervals[i - 1].end > intervals[i].start {
                return false;
            }
        }
        true
    }
}

