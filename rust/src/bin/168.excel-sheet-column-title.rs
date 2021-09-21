/*
 * @lc app=leetcode id=168 lang=rust
 *
 * [168] Excel Sheet Column Title
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn convert_to_title(mut column_number: i32) -> String {
        const TABLE: [&str; 26] = [
            "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q",
            "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
        ];

        let mut ans = String::new();
        while column_number > 0 {
            let n = (column_number - 1) % 26;
            column_number = (column_number - 1) / 26;
            ans = TABLE[n as usize].to_string() + &ans;
        }
        ans
    }
}
// @lc code=end
