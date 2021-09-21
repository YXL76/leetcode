/*
 * @lc app=leetcode id=15 lang=rust
 *
 * [15] 3Sum
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();

        let mut i_now = i32::MAX;
        let mut j_now = i32::MAX;
        let mut ans = Vec::new();
        if nums.len() < 3 {
            return ans;
        }

        for i in 0..nums.len() - 2 {
            if nums[i] == i_now {
                continue;
            } else if i < nums.len() - 3
                && nums[i] == nums[i + 1]
                && nums[i] == nums[i + 2]
                && nums[i] == nums[i + 3]
            {
                continue;
            }
            i_now = nums[i];

            for j in i + 1..nums.len() - 1 {
                if nums[j] == j_now {
                    continue;
                }
                j_now = nums[j];

                let sum = -(nums[i] + nums[j]);
                if nums[j + 1..].binary_search(&sum).is_ok() {
                    ans.push(vec![nums[i], nums[j], sum]);
                }
            }
        }

        ans
    }
}
// @lc code=end
