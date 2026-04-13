impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut res = Vec::new();
        let (mut up, mut left, mut right, mut down) = (false, false, true, false);
        let (mut cap_up, mut cap_left, mut cap_right, mut cap_down) = (0, -1, m as i32, n as i32);
        let (mut i, mut j) = (0i32, 0i32);
        loop {
            if res.len() == n * m {
                break;
            }
            if up {
                if i == cap_up {
                    cap_up += 1;
                    i += 1;
                    j += 1;
                    up = false;
                    right = true;
                } else {
                    res.push(matrix[i as usize][j as usize]);
                    i -= 1;
                }
            } else if left {
                if j == cap_left {
                    cap_left += 1;
                    j += 1;
                    i -= 1;
                    left = false;
                    up = true;
                } else {
                    res.push(matrix[i as usize][j as usize]);
                    j -= 1;
                }
            } else if right {
                if j == cap_right {
                    cap_right -= 1;
                    j -= 1;
                    i += 1;
                    right = false;
                    down = true;
                } else {
                    res.push(matrix[i as usize][j as usize]);
                    j += 1;
                }
            } else if down {
                if i == cap_down {
                    cap_down -= 1;
                    i -= 1;
                    j -= 1;
                    down = false;
                    left = true;
                } else {
                    res.push(matrix[i as usize][j as usize]);
                    i += 1;
                }
            }
        }
        res
    }
}
