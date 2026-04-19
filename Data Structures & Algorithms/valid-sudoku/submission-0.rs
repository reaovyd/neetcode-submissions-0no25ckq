use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let n = board.len();
        let m = board[0].len();

        for i in 0..n {
            let mut set = HashSet::new();
            for j in 0..m {
                if board[i][j] == '.' {
                    continue;
                }
                if !set.contains(&board[i][j]) {
                    set.insert(board[i][j]);
                } else {
                    return false;
                }
            }
        }
        for j in 0..m {
            let mut set = HashSet::new();
            for i in 0..n {
                if board[i][j] == '.' {
                    continue;
                }
                if !set.contains(&board[i][j]) {
                    set.insert(board[i][j]);
                } else {
                    return false;
                }
            }
        }
        for i in (0..n).step_by(3) {
            for j in (0..m).step_by(3) {
                let mut set = HashSet::new();
                for x in 0..3 {
                    for y in 0..3 {
                        let dx = i + x;
                        let dy = j + y;
                        if board[dx][dy] == '.' {
                            continue;
                        }
                        if !set.contains(&board[dx][dy]) {
                            set.insert(board[dx][dy]);
                        } else {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }
}
