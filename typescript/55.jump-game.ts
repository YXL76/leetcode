/*
 * @lc app=leetcode id=55 lang=typescript
 *
 * [55] Jump Game
 */

// @lc code=start
function canJump(nums: number[]): boolean {
    let max = 0;
    for (let i = 0; max >= i && i < nums.length; ++i) {
        const curr = i + nums[i];
        if (curr > max) max = curr;
    }
    return max >= nums.length - 1;
}
// @lc code=end
