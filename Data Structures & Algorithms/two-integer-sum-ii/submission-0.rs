impl Solution {
    pub fn two_sum(mut numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let n = numbers.len();
        let mut i = 0;
        let mut j = n as i32 - 1;
        while i < j {
            let sum = numbers[i as usize] + numbers[j as usize];
            if sum > target {
                j -= 1;
            } else if sum < target {
                i += 1;
            } else {
                return vec![i + 1, j + 1];
            }
        }
        todo!()
    }
}
