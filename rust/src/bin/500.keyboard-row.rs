/*
 * @lc app=leetcode id=500 lang=rust
 *
 * [500] Keyboard Row
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        let mut ans = vec![];

        for j in &['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'] {
            map.insert(*j, 0);
        }
        for j in &['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l'] {
            map.insert(*j, 1);
        }
        for j in &['z', 'x', 'c', 'v', 'b', 'n', 'm'] {
            map.insert(*j, 2);
        }

        'outer: for word in words {
            let mut row = -1;
            for c in word.chars() {
                let l = c.to_ascii_lowercase();
                let v = *map.get(&l).unwrap();
                if row == -1 {
                    row = v;
                } else if v != row {
                    continue 'outer;
                }
            }
            ans.push(word);
        }

        ans
    }
}
// @lc code=end
