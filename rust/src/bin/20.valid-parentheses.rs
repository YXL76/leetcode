/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut count = s.len();
        let mut stack = Vec::new();
        for c in s.chars() {
            let len = stack.len();
            if count < len {
                return false;
            }
            count -= 1;
            if len > 0 {
                let p = stack[len - 1];
                if p == '(' && c == ')' || p == '[' && c == ']' || p == '{' && c == '}' {
                    stack.pop();
                    continue;
                }
            }
            stack.push(c);
        }

        stack.is_empty()
    }
}
// @lc code=end
