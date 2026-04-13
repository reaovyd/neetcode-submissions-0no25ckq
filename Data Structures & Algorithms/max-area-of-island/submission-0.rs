impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut res = 0;
        (0..n).for_each(|i| {
            (0..m).for_each(|j| {
                if grid[i][j] == 1 {
                    res = res.max(Self::dfs(&mut grid, i as i32, j as i32));
                }
            });
        });
        res
    }

    pub fn dfs(grid: &mut [Vec<i32>], i: i32, j: i32) -> i32 {
        if i < 0
            || j < 0
            || i >= grid.len() as i32
            || j >= grid[0].len() as i32
            || grid[i as usize][j as usize] == 0
        {
            return 0;
        }
        grid[i as usize][j as usize] = 0;
        let d1 = Self::dfs(grid, i + 1, j);
        let d2 = Self::dfs(grid, i - 1, j);
        let d3 = Self::dfs(grid, i, j + 1);
        let d4 = Self::dfs(grid, i, j - 1);
        1 + d1 + d2 + d3 + d4
    }
}
