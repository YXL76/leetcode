/*
 * @lc app=leetcode id=191 lang=rust
 *
 * [191] Number of 1 Bits
 */

// @lc code=start
impl Solution {
    pub fn hammingWeight(mut n: u32) -> i32 {
        let mut ans = 0;
        for i in 0..32 {
            ans += n & 1;
            n >>= 1;
            if n == 0 {
                break;
            }
        }
        ans as i32
    }
}
// @lc code=end
