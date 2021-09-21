/*
 * @lc app=leetcode id=200 lang=rust
 *
 * [200] Number of Islands
 */

pub struct Solution;

// @lc code=start
impl Solution {
    const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        use std::collections::VecDeque;

        let m = grid.len();
        let n = grid[0].len();
        let mut queue = VecDeque::new();

        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '0' {
                    continue;
                }
                ans += 1;
                grid[i][j] = '0';
                queue.push_back((i, j));
                while let Some((x, y)) = queue.pop_front() {
                    for (dx, dy) in Self::DIRECTIONS.iter() {
                        let x_ = x as i32 + dx;
                        let y_ = y as i32 + dy;
                        if x_ < 0
                            || x_ >= m as i32
                            || y_ < 0
                            || y_ >= n as i32
                            || grid[x_ as usize][y_ as usize] == '0'
                        {
                            continue;
                        }
                        grid[x_ as usize][y_ as usize] = '0';
                        queue.push_back((x_ as usize, y_ as usize));
                    }
                }
            }
        }

        ans
    }
}
// @lc code=end
