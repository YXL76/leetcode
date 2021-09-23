/*
 * @lc app=leetcode id=722 lang=rust
 *
 * [722] Remove Comments
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn remove_comments(source: Vec<String>) -> Vec<String> {
        let mut flag = false;
        let mut now = String::new();
        let mut ans = vec![];
        for mut s in source {
            if flag {
                if let Some(i) = s.find("*/") {
                    now += unsafe { s.get_unchecked(i + 2..) };
                    s = now.clone();
                    flag = false;
                } else {
                    continue;
                }
            }

            while let Some(i) = s.find("/*") {
                if let Some(j) = s.find("//") {
                    if j < i {
                        s = unsafe { s.get_unchecked(0..j) }.to_string();
                        break;
                    }
                }

                let (first, last) = s.split_at(i + 2);
                if let Some(j) = last.find("*/") {
                    s = unsafe { s.get_unchecked(0..i) }.to_string()
                        + unsafe { s.get_unchecked(i + j + 4..) };
                } else {
                    s = unsafe { s.get_unchecked(0..i) }.to_string();
                    now += s.as_str();
                    flag = true;
                }
            }

            if !flag {
                if let Some(i) = s.find("//") {
                    s = unsafe { s.get_unchecked(0..i) }.to_string();
                }
                ans.push(s);
                now.clear();
            }
        }
        ans.retain(|s| s.len() > 0);
        ans
    }
}
// @lc code=end
