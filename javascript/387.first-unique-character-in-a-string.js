/*
 * @lc app=leetcode id=387 lang=javascript
 *
 * [387] First Unique Character in a String
 */

// @ts-check

// @lc code=start
/**
 * @param {string} s
 * @return {number}
 */
var firstUniqChar = function (s) {
  /** @type {Set<string>} */
  const set = new Set();
  /** @type {Map<string, number>} */
  const map = new Map();
  for (let i = 0; i < s.length; i++) {
    if (set.has(s[i])) {
      map.delete(s[i]);
    } else {
      set.add(s[i]);
      map.set(s[i], i);
    }
  }
  return map.size > 0 ? map.values().next().value : -1;
};
// @lc code=end
