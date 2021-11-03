/*
 * @lc app=leetcode id=54 lang=rust
 *
 * [54] Spiral Matrix
 */

// @lc code=start
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut ans = Vec::new();

        if m == 0 || n == 0 {
            return ans;
        }

        let mut f = 0;
        let mut corner = [(0, 0), (0, n - 1), (m - 1, n - 1), (m - 1, 0)];

        while ans.len() < m * n {
            match f {
                0 => {
                    for i in corner[0].1..=corner[1].1 {
                        ans.push(matrix[corner[0].0][i]);
                    }
                    corner[0].0 += 1;
                    corner[1].0 += 1;
                }
                1 => {
                    for i in corner[1].0..=corner[2].0 {
                        ans.push(matrix[i][corner[1].1]);
                    }
                    corner[1].1 -= 1;
                    corner[2].1 -= 1;
                }
                2 => {
                    for i in (corner[3].1..=corner[2].1).rev() {
                        ans.push(matrix[corner[2].0][i]);
                    }
                    corner[2].0 -= 1;
                    corner[3].0 -= 1;
                }
                3 => {
                    for i in (corner[0].0..=corner[3].0).rev() {
                        ans.push(matrix[i][corner[3].1]);
                    }
                    corner[3].1 += 1;
                    corner[0].1 += 1;
                }
                _ => (),
            }

            f = (f + 1) % 4;
        }

        ans
    }
}
// @lc code=end
