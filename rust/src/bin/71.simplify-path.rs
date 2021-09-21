/*
 * @lc app=leetcode id=71 lang=rust
 *
 * [71] Simplify Path
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack = vec![""];
        let items: Vec<_> = path.split('/').collect();
        for item in items {
            match item {
                "." | "" => (),
                ".." => {
                    if stack.len() > 1 {
                        stack.pop();
                    }
                }
                _ => stack.push(item),
            }
        }
        return if stack.len() == 1 {
            "/".to_string()
        } else {
            stack.join("/")
        };
    }
}
// @lc code=end
