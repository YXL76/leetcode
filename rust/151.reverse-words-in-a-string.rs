/*
 * @lc app=leetcode id=151 lang=rust
 *
 * [151] Reverse Words in a String
 */

// @lc code=start
impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.trim()
            .split(' ')
            .filter(|s| s.len() > 0)
            .rev()
            .collect::<Vec<&str>>()
            .join(" ")
    }
}
// @lc code=end
