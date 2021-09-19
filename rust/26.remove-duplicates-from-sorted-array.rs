/*
 * @lc app=leetcode id=26 lang=rust
 *
 * [26] Remove Duplicates from Sorted Array
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let mut set: HashSet<i32> = nums.iter().cloned().collect();
        *nums = set.iter().cloned().collect();
        nums.sort();
        nums.len() as i32
    }
}
// @lc code=end
