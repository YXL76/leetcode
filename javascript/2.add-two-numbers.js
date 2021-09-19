/*
 * @lc app=leetcode id=2 lang=javascript
 *
 * [2] Add Two Numbers
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} l1
 * @param {ListNode} l2
 * @return {ListNode}
 */
var addTwoNumbers = function (l1, l2) {
  let sum = l1.val + l2.val;
  l1 = l1.next;
  l2 = l2.next;
  let flag = sum >= 10 ? 1 : 0;
  let result = new ListNode(sum % 10);
  let now = result;

  while (l1) {
    if (l2) {
      sum = l1.val + l2.val + flag;
      l2 = l2.next;
    } else {
      sum = l1.val + flag;
    }
    l1 = l1.next;

    flag = sum >= 10 ? 1 : 0;
    now.next = new ListNode(sum % 10);
    now = now.next;
  }

  while (l2) {
    sum = l2.val + flag;
    l2 = l2.next;

    flag = sum >= 10 ? 1 : 0;
    now.next = new ListNode(sum % 10);
    now = now.next;
  }

  if (flag) {
    now.next = new ListNode(1);
  }

  return result;
};
// @lc code=end
