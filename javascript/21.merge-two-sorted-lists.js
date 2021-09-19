/*
 * @lc app=leetcode id=21 lang=javascript
 *
 * [21] Merge Two Sorted Lists
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
var mergeTwoLists = function (l1, l2) {
  if (!l1) return l2;
  if (!l2) return l1;

  const insert = (old, node) => {
    const next = old.next;
    old.next = node;
    node.next = next;
  };

  const list = l1.val < l2.val ? l1 : l2;
  let j = l1.val < l2.val ? l2 : l1;
  let i = list;
  let ii = i.next;

  while (ii && j) {
    while (ii.val < j.val) {
      i = ii;
      ii = ii.next;
      if (!ii) {
        if (j) i.next = j;
        return list;
      }
    }
    tmp = j;
    j = j.next;
    if (ii.val < tmp.val) {
      insert(ii, tmp);
      ii = ii.next;
    } else {
      insert(i, tmp);
    }
    i = i.next;
  }

  if (j) i.next = j;

  return list;
};
// @lc code=end
