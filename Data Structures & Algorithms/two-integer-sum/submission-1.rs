use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let map = nums
            .iter()
            .enumerate()
            .map(|(idx, num)| (*num, idx as i32))
            .collect::<HashMap<i32, i32>>();
        for (idx, num) in nums.into_iter().enumerate() {
            let c = target - num;
            if let Some(idxx) = map.get(&c) {
                if *idxx != idx as i32 {
                    return vec![
                        std::cmp::min(*idxx, idx as i32),
                        std::cmp::max(*idxx, idx as i32),
                    ];
                }
            }
        }
        todo!()
    }
}
