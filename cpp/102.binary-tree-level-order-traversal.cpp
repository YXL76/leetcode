/*
 * @lc app=leetcode id=102 lang=cpp
 *
 * [102] Binary Tree Level Order Traversal
 */

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
  vector<vector<int>> levelOrder(TreeNode *root) {
    vector<vector<int>> res;
    if (root == nullptr) return res;
    res.push_back(vector<int>{root->val});
    int l = 0, r = 1;
    vector<TreeNode *> nodes{root};
    while (true) {
      vector<int> tmp;
      for (; l < r; ++l) {
        TreeNode *left = nodes[l]->left;
        TreeNode *right = nodes[l]->right;
        if (left != nullptr) {
          nodes.push_back(left);
          tmp.push_back(left->val);
        }
        if (right != nullptr) {
          nodes.push_back(right);
          tmp.push_back(right->val);
        }
      }
      int len = tmp.size();
      if (len == 0) return res;
      res.push_back(tmp);
      r += len;
    }
  }
};
// @lc code=end
