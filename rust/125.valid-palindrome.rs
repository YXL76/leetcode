/*
 * @lc app=leetcode id=125 lang=rust
 *
 * [125] Valid Palindrome
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let v: Vec<char> = s
            .chars()
            .filter_map(|mut c| {
                if c.is_ascii_alphanumeric() {
                    c.make_ascii_lowercase();
                    Some(c)
                } else {
                    None
                }
            })
            .collect();

        if v.len() == 0 {
            return true;
        }

        let mut l = 0;
        let mut r = v.len() - 1;
        while l < r {
            if v[l] != v[r] {
                return false;
            }
            l += 1;
            r -= 1;
        }

        true
    }
}
// @lc code=end
