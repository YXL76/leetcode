/*
 * @lc app=leetcode id=171 lang=typescript
 *
 * [171] Excel Sheet Column Number
 */

// @lc code=start
function titleToNumber(columnTitle: string): number {
    let ans = 0;
    let cur = 1;
    for (let i = columnTitle.length - 1; i >= 0; --i) {
        const char = columnTitle.charCodeAt(i) - 64;
        ans += cur * char;
        cur *= 26;
    }
    return ans;
}
// @lc code=end
