/*
 * @lc app=leetcode id=91 lang=rust
 *
 * [91] Decode Ways
 */

// @lc code=start
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let nums = s
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
        let len = nums.len();
        if len == 0 || nums[0] == 0 {
            return 0;
        }

        let mut dp = vec![0; len + 1];
        dp[0] = 1;
        dp[1] = 1;

        for i in 2..=len {
            if nums[i - 1] != 0 {
                dp[i] += dp[i - 1];
            }
            if nums[i - 2] != 0 && nums[i - 2] * 10 + nums[i - 1] <= 26 {
                dp[i] += dp[i - 2];
            }
        }

        dp[len]
    }
}
// @lc code=end
