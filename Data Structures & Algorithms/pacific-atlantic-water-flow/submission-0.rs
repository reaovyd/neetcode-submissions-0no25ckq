use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = heights.len();
        let m = heights[0].len();
        let mut pacific = HashSet::new();
        let mut atlantic = HashSet::new();

        for i in 0..n {
            let (row, col) = (i, 0);
            Self::_marker(&heights, &mut pacific, n, m, row, col);
        }
        for j in 0..m {
            let (row, col) = (0, j);
            Self::_marker(&heights, &mut pacific, n, m, row, col);
        }
        for i in 0..n {
            let (row, col) = (i, m - 1);
            Self::_marker(&heights, &mut atlantic, n, m, row, col);
        }
        for j in 0..m {
            let (row, col) = (n - 1, j);
            Self::_marker(&heights, &mut atlantic, n, m, row, col);
        }
        atlantic.intersection(&pacific).cloned().map(|(x, y)| vec![x, y]).collect::<Vec<Vec<i32>>>()
    }

    pub fn _marker(
        heights: &[Vec<i32>],
        set: &mut HashSet<(i32, i32)>,
        n: usize,
        m: usize,
        st_x: usize,
        st_y: usize,
    ) {
        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        let mut queue = VecDeque::new();
        let mut visited = vec![vec![false; m]; n];
        visited[st_x][st_y] = true;
        set.insert((st_x as i32, st_y as i32));
        queue.push_front((st_x, st_y));
        while !queue.is_empty() {
            let (x, y) = queue.pop_front().unwrap();
            let (x, y) = (x as i32, y as i32);
            for (dx, dy) in directions {
                let dx = x + dx;
                let dy = y + dy;
                if dx >= n as i32
                    || dx < 0
                    || dy >= m as i32
                    || dy < 0
                    || visited[dx as usize][dy as usize]
                    || heights[x as usize][y as usize] > heights[dx as usize][dy as usize]
                {
                    continue;
                }
                queue.push_front((dx as usize, dy as usize));
                visited[dx as usize][dy as usize] = true;
                set.insert((dx, dy));
            }
        }
    }
}
