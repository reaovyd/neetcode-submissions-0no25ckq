impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let idx = search_column(&matrix, target);
        matrix[idx as usize].binary_search(&target).is_ok()
    }
}
pub fn search_column(matrix: &[Vec<i32>], target: i32) -> i32 {
    let mut i = 0;
    let mut j = matrix.len() as i32;
    while i < j {
        let m = (i + j) >> 1;
        if target >= matrix[m as usize][0] {
            if i == m {
                break;
            }
            i = m;
        } else {
            j = m;
        }
    }
    i
}
