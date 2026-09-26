impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let m = matrix[0].len();

        let mut rows = vec![false; n];
        let mut cols = vec![false; m];



        for i in 0..n {
            for j in 0..m {
                if matrix[i][j] == 0 {
                    rows[i] = true;
                    cols[j] = true;
                }
            }
        }

        for i in 0..n {
            if rows[i] {
                for j in 0..m {
                    matrix[i][j] = 0;
                }
            }
        }

        for j in 0..m {
            if cols[j] {
                for i in 0..n {
                    matrix[i][j] = 0;
                }
            }
        }
    }
}