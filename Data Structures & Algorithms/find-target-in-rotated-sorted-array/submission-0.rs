impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let i = search_min(&nums);
        if let Ok(fh) = nums[0..i].binary_search(&target) {
            return fh as i32;
        }
        if let Ok(rh) = nums[i..].binary_search(&target) {
            return (rh + i) as i32;
        }
        -1
    }
}

fn search_min(nums: &[i32]) -> usize {
    let n = nums.len();
    let (mut i, mut j) = (0, n - 1);
    while i < j {
        let m = (j + i) >> 1;
        if nums[m] > nums[j] {
            i = m + 1;
        } else {
            j = m;
        }
    }

    i
}
