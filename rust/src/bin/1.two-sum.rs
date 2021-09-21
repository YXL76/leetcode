/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        for i in 0..nums.len() {
            if let Some(&v) = map.get(&(target - nums[i])) {
                return vec![v as i32, i as i32];
            }
            map.insert(nums[i], i);
        }

        vec![0, 0]
    }
}
// @lc code=end
