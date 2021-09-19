/*
 * @lc app=leetcode id=24 lang=javascript
 *
 * [24] Swap Nodes in Pairs
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
 * @param {ListNode} head
 * @return {ListNode}
 */
var swapPairs = function (head) {
  if (!head || !head.next) return head;
  let i = head;
  let j = head.next;
  {
    const j_next = j.next;
    j.next = i;
    i.next = j_next;
  }
  head = j;
  j = i.next;
  let k = j?.next;
  while (j && k) {
    const k_next = k.next;
    i.next = k;
    k.next = j;
    j.next = k_next;
    i = j;
    j = j.next;
    k = j?.next;
  }
  return head;
};
// @lc code=end
