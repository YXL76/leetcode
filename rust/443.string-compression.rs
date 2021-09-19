/*
 * @lc app=leetcode id=443 lang=rust
 *
 * [443] String Compression
 */

// @lc code=start
impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        if chars.len() <= 1 {
            return chars.len() as i32;
        }
        let mut v = Vec::new();
        let mut c = chars[0];
        let mut count = 1;
        for i in 1..chars.len() {
            if chars[i] == c {
                count += 1;
                continue;
            }
            v.push(c);
            c = chars[i];
            if count > 1 {
                for n in count.to_string().chars() {
                    v.push(n);
                }
            }
            count = 1;
        }
        if count > 0 {
            v.push(c);
            if count > 1 {
                for n in count.to_string().chars() {
                    v.push(n);
                }
            }
        }
        *chars = v;
        chars.len() as i32
    }
}
// @lc code=end
