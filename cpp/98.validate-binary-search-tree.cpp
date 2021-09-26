/*
 * @lc app=leetcode id=98 lang=cpp
 *
 * [98] Validate Binary Search Tree
 */

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
#include <limits>

class Solution {
 public:
  bool isValidBST(TreeNode *root) {
    return isValidNode(root, std::numeric_limits<long long>::max(),
                       std::numeric_limits<long long>::min());
  }

 private:
  bool isValidNode(TreeNode *root, long long max, long long min) {
    if (root == nullptr) return true;
    if (root->val <= min || root->val >= max) return false;
    return isValidNode(root->left, root->val, min) &&
           isValidNode(root->right, max, root->val);
  }
};
// @lc code=end
