/*
 * @lc app=leetcode id=138 lang=cpp
 *
 * [138] Copy List with Random Pointer
 */

class Node {
 public:
  int val;
  Node* next;
  Node* random;

  Node(int _val) {
    val = _val;
    next = nullptr;
    random = nullptr;
  }
};

// @lc code=start
/*
// Definition for a Node.
class Node {
public:
    int val;
    Node* next;
    Node* random;

    Node(int _val) {
        val = _val;
        next = NULL;
        random = NULL;
    }
};
*/

class Solution {
 public:
  Node* copyRandomList(Node* head) {
    if (!head) return nullptr;

    auto h = head;
    while (h) {
      auto ins = new Node(h->val);
      ins->next = h->next;
      h->next = ins;
      h = ins->next;
    }

    h = head;
    while (h) {
      if (h->random) h->next->random = h->random->next;
      h = h->next->next;
    }

    h = head;
    auto ans = h->next;
    auto cur = ans;
    h->next = h->next->next;
    h = h->next;
    while (h) {
      cur->next = h->next;
      cur = cur->next;
      h->next = h->next->next;
      h = h->next;
    };

    return ans;
  }
};
// @lc code=end
