impl Solution {
    pub fn solve(board: &mut [Vec<char>]) {
        let n = board.len();
        let m = board[0].len();
        let mut visited = vec![vec![false; m]; n];

        (0..n).for_each(|i| {
            (0..m).for_each(|j| {
                if board[i][j] == 'O' && !visited[i][j] {
                    let mut vec_updates = Vec::new();
                    let cond = dfs(
                        board,
                        n as i32,
                        m as i32,
                        i as i32,
                        j as i32,
                        &mut visited,
                        &mut vec_updates,
                    );
                    if !cond {
                        for update in vec_updates {
                            board[update.0][update.1] = 'X';
                        }
                    }
                }
            });
        });
    }
}

pub fn dfs(
    board: &[Vec<char>],
    n: i32,
    m: i32,
    i: i32,
    j: i32,
    visited: &mut [Vec<bool>],
    vec: &mut Vec<(usize, usize)>,
) -> bool {
    if i >= n
        || j >= m
        || i < 0
        || j < 0
        || visited[i as usize][j as usize]
        || board[i as usize][j as usize] == 'X'
    {
        return false;
    }
    visited[i as usize][j as usize] = true;
    vec.push((i as usize, j as usize));
    let down = dfs(board, n, m, i + 1, j, visited, vec);
    let up = dfs(board, n, m, i - 1, j, visited, vec);
    let right = dfs(board, n, m, i, j + 1, visited, vec);
    let left = dfs(board, n, m, i, j - 1, visited, vec);
    i == 0 || j == 0 || i == n - 1 || j == m - 1 || down || up || right || left
}
