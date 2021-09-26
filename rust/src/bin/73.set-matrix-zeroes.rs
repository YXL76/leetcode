/*
 * @lc app=leetcode id=73 lang=rust
 *
 * [73] Set Matrix Zeroes
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        use std::collections::HashSet;

        let m = matrix.len();
        let n = matrix[0].len();
        let mut row = HashSet::new();
        let mut col = HashSet::new();
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    row.insert(i);
                    col.insert(j);
                }
            }
        }
        for i in row {
            for j in 0..n {
                matrix[i][j] = 0;
            }
        }
        for i in col {
            for j in 0..m {
                matrix[j][i] = 0;
            }
        }
    }
}
// @lc code=end
