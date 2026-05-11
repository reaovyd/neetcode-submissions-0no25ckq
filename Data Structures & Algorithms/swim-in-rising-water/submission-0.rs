impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut j = n * n;
        let mut i = 0;
        let mut visited = vec![vec![false; n]; n];
        let mut ans = j;
        while i <= j {
            let m = (i + j) >> 1;
            let found = dfs(&grid, 0, 0, &mut visited, m);
            visited.iter_mut().for_each(|item| {
                item.iter_mut().for_each(|item| {
                    *item = false;
                });
            });
            if found {
                ans = ans.min(m);
                j = m - 1;
            } else {
                i = m + 1;
            }
        }
        ans as i32
    }
}
fn dfs(grid: &[Vec<i32>], i: i32, j: i32, visited: &mut [Vec<bool>], time: usize) -> bool {
    if i as usize >= grid.len()
        || j as usize >= grid[0].len()
        || i < 0
        || j < 0
        || visited[i as usize][j as usize]
        || grid[i as usize][j as usize] > time as i32
    {
        return false;
    }
    if i == (grid.len() - 1) as i32 && j == (grid[0].len() - 1) as i32 {
        return true;
    }
    visited[i as usize][j as usize] = true;
    let d1 = dfs(grid, i + 1, j, visited, time);
    let d2 = dfs(grid, i - 1, j, visited, time);
    let d3 = dfs(grid, i, j + 1, visited, time);
    let d4 = dfs(grid, i, j - 1, visited, time);
    d1 || d2 || d3 || d4
}
