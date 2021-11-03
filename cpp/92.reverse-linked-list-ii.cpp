/*
 * @lc app=leetcode id=92 lang=cpp
 *
 * [92] Reverse Linked List II
 */

struct ListNode {
  int val;
  ListNode* next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode* next) : val(x), next(next) {}
};

// @lc code=start
/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution {
 public:
  ListNode* reverseBetween(ListNode* head, int left, int right) {
    auto head_prev = new ListNode(0, head);

    auto cnt = 0;
    auto cur = head_prev;
    while (++cnt < left) {
      cur = cur->next;
    }

    auto left_prev = cur;
    auto left_cur = cur->next;

    if (cnt < right) {
      auto prev = cur->next;
      cur = prev->next;

      while (++cnt <= right) {
        auto next = cur->next;
        cur->next = prev;
        prev = cur;
        cur = next;
      }

      left_prev->next = prev;
      left_cur->next = cur;
    }

    return head_prev->next;
  }
};
// @lc code=end
