/*
 * @lc app=leetcode id=33 lang=rust
 *
 * [33] Search in Rotated Sorted Array
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0 as i32;
        let mut r = nums.len() as i32 - 1;
        let right = nums[r as usize];

        if target <= nums[r as usize] {
            while l <= r {
                let m = l + (r - l) / 2;
                if nums[m as usize] == target {
                    return m as i32;
                }
                if nums[m as usize] < target {
                    l = m + 1;
                } else {
                    if nums[m as usize] > right {
                        l = m + 1;
                    } else {
                        r = m - 1;
                    }
                }
            }
        } else {
            while l <= r {
                let m = l + (r - l) / 2;
                if nums[m as usize] == target {
                    return m as i32;
                }
                if nums[m as usize] > target {
                    r = m - 1;
                } else {
                    if nums[m as usize] < right {
                        r = m - 1;
                    } else {
                        l = m + 1;
                    }
                }
            }
        }

        -1
    }
}
// @lc code=end
