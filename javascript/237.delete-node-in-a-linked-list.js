/*
 * @lc app=leetcode id=237 lang=javascript
 *
 * [237] Delete Node in a Linked List
 */

// @ts-check
/**
 * @param {number} val
 */
function ListNode(val) {
  this.val = val;
  this.next = null;
}

// @lc code=start
/**
 * Definition for singly-linked list.
 * function ListNode(val) {
 *     this.val = val;
 *     this.next = null;
 * }
 */
/**
 * @param {ListNode} node
 * @return {void} Do not return anything, modify node in-place instead.
 */
var deleteNode = function (node) {
  let next = node.next;
  node.val = next.val;
  node.next = next.next;
};
// @lc code=end
