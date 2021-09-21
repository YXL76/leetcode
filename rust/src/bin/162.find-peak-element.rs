/*
 * @lc app=leetcode id=162 lang=rust
 *
 * [162] Find Peak Element
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 1 {
            return 0;
        }

        let mut left = 0;
        let mut right = len - 1;
        while right - left > 1 {
            let mid = left + (right - left) / 2;
            if nums[mid] < nums[mid + 1] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        if left == len - 1 || nums[left] > nums[left + 1] {
            left as i32
        } else {
            right as i32
        }
    }
}
// @lc code=end
