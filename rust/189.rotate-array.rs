/*
 * @lc app=leetcode id=189 lang=rust
 *
 * [189] Rotate Array
 */

// @lc code=start
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize % nums.len();
        let j = nums.len() - k;
        if j == 0 || k == 0 {
            return;
        }
        if k < j {
            let dst = {
                let (left, right) = nums.split_at_mut(j);
                let mut dst = vec![0; j];
                dst.copy_from_slice(&left);
                left[..k].copy_from_slice(&right);
                dst
            };
            nums[k..].copy_from_slice(&dst);
        } else {
            let dst = {
                let (left, right) = nums.split_at_mut(j);
                let mut dst = vec![0; k];
                dst.copy_from_slice(&right);
                right[k - j..].copy_from_slice(&left);
                dst
            };
            nums[..k].copy_from_slice(&dst);
        };
    }
}
// @lc code=end
