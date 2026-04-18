use std::collections::VecDeque;

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();

        let mut queue = VecDeque::new();
        let mut co = 0;
        (0..n).for_each(|i| {
            (0..m).for_each(|j| {
                if grid[i][j] == 2 {
                    queue.push_front((i, j));
                } else if grid[i][j] == 1 {
                    co += 1;
                }
            });
        });

        let mut mins = 0;
        while !queue.is_empty() {
            let sz = queue.len();
            let mut rm = false;
            for _ in 0..sz {
                let (x, y) = queue.pop_back().unwrap();
                for (dx, dy) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
                    let dx = x as i32 + dx;
                    let dy = y as i32 + dy;
                    if dx < 0
                        || dy < 0
                        || dx >= n as i32
                        || dy >= m as i32
                        || grid[dx as usize][dy as usize] == 2
                        || grid[dx as usize][dy as usize] != 1
                    {
                        continue;
                    }
                    rm = true;
                    grid[dx as usize][dy as usize] = 2;
                    queue.push_front((dx as usize, dy as usize));
                    co -= 1;
                }
            }
            if rm {
                mins += 1;
            }
        }
        if co != 0 {
            return -1;
        }

        mins
    }
}
