impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        transpose(matrix);
        reflect(matrix);
    }
}

fn transpose(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();
    (0..n).for_each(|i| {
        (i..n).for_each(|j| {
            let tmp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = tmp;
        });
    });
}

fn reflect(matrix: &mut [Vec<i32>]) {
    let n = matrix.len();
    (0..n).for_each(|i| {
        let mut j = 0;
        let mut m = matrix.len() - 1;
        while j < m {
            matrix[i].swap(m, j);
            j += 1;
            m -= 1;
        }
    });
}
