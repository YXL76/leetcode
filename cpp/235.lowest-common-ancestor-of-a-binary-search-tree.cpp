/*
 * @lc app=leetcode id=235 lang=cpp
 *
 * [235] Lowest Common Ancestor of a Binary Search Tree
 */

struct TreeNode {
  int val;
  TreeNode* left;
  TreeNode* right;
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};

// @lc code=start
/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */

class Solution {
 public:
  TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) {
    if (root == nullptr) return root;
    int i = p->val;
    int j = q->val;
    if (i > j) {
      i ^= j;
      j ^= i;
      i ^= j;
    }
    TreeNode* curr = root;
    while (curr != nullptr) {
      const int val = curr->val;
      if (val == i || val == j) return curr;
      if (val > i) {
        if (val < j) return curr;
        curr = curr->left;
      } else
        curr = curr->right;
    }
    return root;
  }
};
// @lc code=end
