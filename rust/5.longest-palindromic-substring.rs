/*
 * @lc app=leetcode id=5 lang=rust
 *
 * [5] Longest Palindromic Substring
 */

// @lc code=start
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let len = s.len();
        let v = s.as_bytes();
        let mut d = vec![vec![false; len]; len];
        let mut ans = (0, 0);

        for i in 0..len {
            d[i][i] = true;
        }

        for i in 1..len {
            if v[i - 1] == v[i] {
                ans = (i - 1, i);
                d[i - 1][i] = true;
            };
        }

        for i in 2..len {
            for j in i..len {
                if v[j - i] == v[j] && d[j - i + 1][j - 1] {
                    ans = (j - i, j);
                    d[j - i][j] = true;
                }
            }
        }

        unsafe { s.get_unchecked(ans.0..=ans.1) }.to_string()
    }
}
// @lc code=end
