/*
 * @lc app=leetcode id=213 lang=rust
 *
 * [213] House Robber II
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 1 {
            return nums[0];
        }

        let mut ans = 0;
        let mut d = vec![vec![0; len]; len];
        for i in 0..len {
            d[i][i] = nums[i];
        }
        for i in 1..len {
            d[i - 1][i] = nums[i - 1].max(nums[i]);
            if d[i - 1][i] > ans {
                ans = d[i - 1][i];
            }
        }

        for i in 2..len - 1 {
            for j in 0..len - i {
                d[j][j + i] = (nums[j] + d[j + 2][j + i])
                    .max(d[j][j + i - 2] + nums[j + i])
                    .max(d[j + 1][j + i - 1]);
                if d[j][j + i] > ans {
                    ans = d[j][j + i];
                }
            }
        }

        ans
    }
}
// @lc code=end
