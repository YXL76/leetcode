/*
 * @lc app=leetcode id=5 lang=rust
 *
 * [5] Longest Palindromic Substring
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let len = s.len();
        let v = s.as_bytes();
        let mut d = vec![0; len];
        let mut ans = 0;

        for i in 1..len {
            if v[i - 1] == v[i] {
                d[i] = d[i - 1];
            } else {
                d[i] = i;
                let p = i - 1;
                if p - d[p] > ans - d[ans] {
                    ans = p;
                }
            }
        }
        {
            let p = len - 1;
            if p - d[p] > ans - d[ans] {
                ans = p;
            }

            let mut tmp_ans = 0;
            for i in 2..len {
                for j in (i..len).rev() {
                    let l = j - i;
                    if v[l] == v[j] && d[j - 1] == l + 1 {
                        d[j] = l;
                        tmp_ans = j;
                    }
                }
            }

            if tmp_ans - d[tmp_ans] > ans - d[ans] {
                ans = tmp_ans;
            }
        }

        unsafe { s.get_unchecked(d[ans]..=ans) }.to_string()
    }
}
// @lc code=end
