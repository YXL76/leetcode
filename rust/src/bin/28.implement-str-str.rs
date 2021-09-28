/*
 * @lc app=leetcode id=28 lang=rust
 *
 * [28] Implement strStr()
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let m = needle.len();
        if m == 0 {
            return 0;
        }
        let n = haystack.len();
        if m > n {
            return -1;
        }
        let haystack = haystack.as_bytes();
        let needle = needle.as_bytes();
        'outer: for i in 0..n - m + 1 {
            if haystack[i] == needle[0] {
                for j in 1..m {
                    if haystack[i + j] != needle[j] {
                        continue 'outer;
                    }
                }
                return i as i32;
            }
        }
        -1
    }
}
// @lc code=end
