/*
 * @lc app=leetcode id=114 lang=cpp
 *
 * [114] Flatten Binary Tree to Linked List
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
class Solution {
 public:
  void flatten(TreeNode *root) {
    if (root == nullptr) return;
    TreeNode *h = head;
    traversal(root);
    root->left = nullptr;
    root->right = h->right->right;
  }

 private:
  TreeNode *head = new TreeNode();

  void traversal(TreeNode *root) {
    if (root == nullptr) return;
    head->right = new TreeNode(root->val);
    head = head->right;
    traversal(root->left);
    traversal(root->right);
  }
};
// @lc code=end
