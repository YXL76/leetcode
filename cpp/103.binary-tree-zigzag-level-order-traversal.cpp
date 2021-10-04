/*
 * @lc app=leetcode id=103 lang=cpp
 *
 * [103] Binary Tree Zigzag Level Order Traversal
 */

#include <queue>
#include <vector>
using namespace std;

struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right)
      : val(x), left(left), right(right) {}
};

// @lc code=start
/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left),
 * right(right) {}
 * };
 */
class Solution {
 public:
  vector<vector<int>> zigzagLevelOrder(TreeNode *root) {
    vector<vector<int>> ans;
    if (!root) return ans;
    deque<TreeNode *> q;
    q.push_back(root);
    auto flag = false;

    while (!q.empty()) {
      flag = !flag;
      auto size = q.size();
      vector<int> level;
      if (flag) {
        for (size_t i = 0; i < size; ++i) {
          auto node = q.front();
          q.pop_front();
          level.push_back(node->val);
          if (node->left) q.push_back(node->left);
          if (node->right) q.push_back(node->right);
        }
      } else {
        for (size_t i = 0; i < size; ++i) {
          auto node = q.back();
          q.pop_back();
          level.push_back(node->val);
          if (node->right) q.push_front(node->right);
          if (node->left) q.push_front(node->left);
        }
      }
      ans.push_back(level);
    }

    return ans;
  }
};
// @lc code=end
