/*
 * @lc app=leetcode id=300 lang=rust
 *
 * [300] Longest Increasing Subsequence
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut ans = 1;
        let mut d = vec![1; len];
        for i in 1..len {
            for j in 0..i {
                if nums[i] > nums[j] && d[i] < d[j] + 1 {
                    d[i] = d[j] + 1;
                    if d[i] > ans {
                        ans = d[i];
                    }
                }
            }
        }
        ans
    }
}
// @lc code=end
