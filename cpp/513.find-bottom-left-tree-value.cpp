/*
 * @lc app=leetcode id=513 lang=cpp
 *
 * [513] Find Bottom Left Tree Value
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
  int findBottomLeftValue(TreeNode *root) {
    dfs(root, 1);
    return ans;
  }

 private:
  int ans = 0;
  int maxd = 0;
  void dfs(TreeNode *root, int d) {
    if (root == nullptr) return;
    if (d > maxd) {
      maxd = d;
      ans = root->val;
    }
    dfs(root->left, d + 1);
    dfs(root->right, d + 1);
  }
};
// @lc code=end
