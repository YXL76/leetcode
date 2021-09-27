/*
 * @lc app=leetcode id=566 lang=rust
 *
 * [566] Reshape the Matrix
 */

// @lc code=start
impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let r = r as usize;
        let c = c as usize;
        let m = mat.len();
        let n = mat[0].len();
        if m * n != r * c {
            return mat;
        }
        let mut ans = vec![vec![0; c]; r];
        let mut x = 0;
        let mut y = 0;
        for i in 0..m {
            for j in 0..n {
                ans[x][y] = mat[i][j];
                if y == c - 1 {
                    x += 1;
                    y = 0;
                } else {
                    y += 1;
                }
            }
        }
        ans
    }
}
// @lc code=end
