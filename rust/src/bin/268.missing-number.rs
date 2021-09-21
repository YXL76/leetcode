/*
 * @lc app=leetcode id=268 lang=rust
 *
 * [268] Missing Number
 */

// @lc code=start
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let sum = nums.iter().sum::<i32>();
        let len = nums.len() as i32;
        (len + 1) * len / 2 - sum
    }
}
// @lc code=end
