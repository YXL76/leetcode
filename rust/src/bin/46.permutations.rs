/*
 * @lc app=leetcode id=46 lang=rust
 *
 * [46] Permutations
 */

pub struct Solution;

// @lc code=start

fn solve(
    nums: &Vec<i32>,
    mut ans: &mut Vec<Vec<i32>>,
    mut vis: &mut Vec<bool>,
    k: usize,
    cur: &mut Vec<i32>,
) {
    if k == 0 {
        ans.push(cur.clone());
        return;
    }
    for i in 0..nums.len() {
        if !vis[i] {
            vis[i] = true;
            let mut cur = cur.clone();
            cur.push(nums[i]);
            solve(&nums, &mut ans, &mut vis, k - 1, &mut cur);
            vis[i] = false;
        }
    }
}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut vis = vec![false; nums.len()];
        let mut cur = Vec::new();
        solve(&nums, &mut ans, &mut vis, nums.len(), &mut cur);
        ans
    }
}
// @lc code=end
