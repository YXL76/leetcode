/*
 * @lc app=leetcode id=567 lang=typescript
 *
 * [567] Permutation in String
 */

// @lc code=start
function checkInclusion(s1: string, s2: string): boolean {
    const s1Len = s1.length;
    const s2Len = s2.length;
    if (s1Len > s2Len) return false;

    const map = new Map<string, number>();
    for (let c of s1) {
        const n = map.get(c);
        map.set(c, typeof n === "number" ? n + 1 : 1);
    }
    for (let i = 0; i < s1Len; ++i) {
        const c = s2[i];
        const n = map.get(c);
        if (typeof n !== "number") map.set(c, -1);
        else if (n === 1) map.delete(c);
        else map.set(c, n - 1);
    }
    if (map.size === 0) return true;

    for (let i = 0; i < s2Len - s1Len; ++i) {
        {
            const c = s2[i];
            const n = map.get(c);
            if (typeof n !== "number") map.set(c, 1);
            else if (n === -1) map.delete(c);
            else map.set(c, n + 1);
        }

        {
            const c = s2[i + s1Len];
            const n = map.get(c);
            if (typeof n !== "number") map.set(c, -1);
            else if (n === 1) map.delete(c);
            else map.set(c, n - 1);
        }

        if (map.size === 0) return true;
    }
    return false;
}
// @lc code=end
