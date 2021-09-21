/*
 * @lc app=leetcode id=13 lang=rust
 *
 * [13] Roman to Integer
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut ans = 0;
        let mut now = 0;
        let mut count = 0;
        for c in s.chars() {
            let n = match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            };

            if count == 0 {
                now = n;
                count = 1;
                continue;
            }

            if n == now {
                count += 1;
                continue;
            }

            if match n {
                5 | 10 if now == 1 => true,
                50 | 100 if now == 10 => true,
                500 | 1000 if now == 100 => true,
                _ => false,
            } {
                ans += n - now;
                count = 0;
                continue;
            }

            ans += now * count;
            now = n;
            count = 1;
        }

        ans + now * count
    }
}
// @lc code=end
