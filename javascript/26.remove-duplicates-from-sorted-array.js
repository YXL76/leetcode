/*
 * @lc app=leetcode id=26 lang=javascript
 *
 * [26] Remove Duplicates from Sorted Array
 */

// @ts-check

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number}
 */
var removeDuplicates = function (nums) {
  const set = new Set(nums);
  let i = 0;
  set.forEach((v) => (nums[i++] = v));
  return set.size;
};
// @lc code=end
