impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        nums.sort();
        let n = nums.len();
        let mut i = 0;
        while i < n {
            let mut j = i + 1;
            let mut k = n - 1;
            while j < k {
                let sm = nums[i] + nums[j] + nums[k];
                if sm > 0 {
                    k -= 1
                } else if sm < 0 {
                    j += 1
                } else {
                    let vc = vec![nums[i], nums[j], nums[k]];
                    ans.push(vc);
                    j += 1;
                    while j < k && nums[j] == nums[j - 1] {
                        j += 1;
                    }
                }
            }
            i += 1;
            while i < n && nums[i] == nums[i - 1] {
                i += 1;
            }
        }
        ans
    }
}
