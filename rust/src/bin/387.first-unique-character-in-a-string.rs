/*
 * @lc app=leetcode id=387 lang=rust
 *
 * [387] First Unique Character in a String
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        use std::collections::{BTreeMap, HashSet};

        let len = s.len();
        let bytes = s.as_bytes();
        let mut set = HashSet::new();
        let mut map = BTreeMap::new();

        for i in 0..len {
            if set.contains(&bytes[i]) {
                map.remove(&bytes[i]);
            } else {
                set.insert(bytes[i]);
                map.insert(bytes[i], i);
            }
        }

        if let Some((_, &i)) = map.iter().next() {
            i as i32
        } else {
            -1
        }
    }
}
// @lc code=end
