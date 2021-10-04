/*
 * @lc app=leetcode id=165 lang=typescript
 *
 * [165] Compare Version Numbers
 */

// @lc code=start
function compareVersion(version1: string, version2: string): number {
    let v1 = version1.split(".");
    let v2 = version2.split(".");
    let i = 0;
    while (i < v1.length && i < v2.length) {
        let n1 = parseInt(v1[i]);
        let n2 = parseInt(v2[i]);
        if (n1 > n2) return 1;
        else if (n1 < n2) return -1;
        ++i;
    }
    while (i < v1.length) {
        if (parseInt(v1[i]) > 0) return 1;
        ++i;
    }
    while (i < v2.length) {
        if (parseInt(v2[i]) > 0) return -1;
        ++i;
    }
    return 0;
}
// @lc code=end
