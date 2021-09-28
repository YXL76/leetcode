/*
 * @lc app=leetcode id=23 lang=cpp
 *
 * [23] Merge k Sorted Lists
 */

#include <queue>
#include <vector>
using namespace std;

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
  ListNode* mergeKLists(vector<ListNode*>& lists) {
    auto cmp = [](ListNode* left, ListNode* right) {
      return left->val > right->val;
    };
    priority_queue<ListNode*, vector<ListNode*>, decltype(cmp)> q(cmp);
    for (auto& list : lists) {
      if (list != nullptr) q.push(list);
    }
    ListNode* head = new ListNode();
    ListNode* curr = head;
    while (!q.empty()) {
      auto node = q.top();
      q.pop();
      curr->next = node;
      curr = curr->next;
      if (node->next != nullptr) q.push(node->next);
    }
    return head->next;
  }
};
// @lc code=end
