impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut res = 0;
        (0..n).for_each(|i| {
            (0..m).for_each(|j| {
                if grid[i][j] == '1' {
                    res += 1;
                    Self::dfs(&mut grid, i as i32, j as i32);
                }
            });
        });
        res
    }

    pub fn dfs(grid: &mut [Vec<char>], i: i32, j: i32) {
        if i < 0
            || j < 0
            || i >= grid.len() as i32
            || j >= grid[0].len() as i32
            || grid[i as usize][j as usize] == '0'
        {
            return;
        }
        grid[i as usize][j as usize] = '0';
        Self::dfs(grid, i + 1, j);
        Self::dfs(grid, i - 1, j);
        Self::dfs(grid, i, j + 1);
        Self::dfs(grid, i, j - 1);
    }
}
