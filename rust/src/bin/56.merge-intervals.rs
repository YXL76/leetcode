/*
 * @lc app=leetcode id=56 lang=rust
 *
 * [56] Merge Intervals
 */

// @lc code=start
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::cmp::Ordering;

        let len = intervals.len();
        let mut ans = Vec::new();

        intervals.sort_by(|a, b| {
            let ord = a[0].cmp(&b[0]);
            match ord {
                Ordering::Equal => b[1].cmp(&a[1]),
                _ => ord,
            }
        });

        let mut curr = intervals[0].clone();
        for i in 1..len {
            if intervals[i][0] <= curr[1] {
                if intervals[i][1] > curr[1] {
                    curr[1] = intervals[i][1];
                }
            } else {
                ans.push(curr);
                curr = intervals[i].clone();
            }
        }

        ans.push(curr);

        ans
    }
}
// @lc code=end
