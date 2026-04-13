impl Solution {
    pub fn islands_and_treasure(grid: &mut Vec<Vec<i32>>) {
        let (n, m) = (grid.len() as i32, grid[0].len() as i32);
        (0..n).for_each(|i| {
            (0..m).for_each(|j| {
                if grid[i as usize][j as usize] == 0 {
                    Self::calculate_mins(grid, 0, i, j, n, m);
                }
            });
        });
    }

    fn calculate_mins(
        grid: &mut [Vec<i32>],
        move_count: i32,
        i: i32,
        j: i32,
        n: i32,
        m: i32,
    ) {
        if i < 0 || j < 0 || i >= n || j >= m {
            return;
        }
        let ii = i as usize;
        let jj = j as usize;
        if grid[ii][jj] == -1 {
            return;
        }
        if move_count > grid[ii][jj] {
            return;
        }

        grid[ii][jj] = min(grid[ii][jj], move_count);
        Self::calculate_mins(grid, move_count + 1, i + 1, j, n, m);
        Self::calculate_mins(grid, move_count + 1, i - 1, j, n, m);
        Self::calculate_mins(grid, move_count + 1, i, j - 1, n, m);
        Self::calculate_mins(grid, move_count + 1, i, j + 1, n, m);
    }
}
