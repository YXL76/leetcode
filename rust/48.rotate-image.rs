/*
 * @lc app=leetcode id=48 lang=rust
 *
 * [48] Rotate Image
 */

// @lc code=start
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let mut v = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                v[i][j] = matrix[n - 1 - j][i];
            }
        }
        *matrix = v;
    }
}
// @lc code=end
