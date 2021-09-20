/*
 * @lc app=leetcode id=650 lang=rust
 *
 * [650] 2 Keys Keyboard
 */

// @lc code=start
impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        if n == 1 {
            return 0;
        }
        const list: [usize; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        list.reverse();
        let n = n as usize;
        let mut v = vec![0; n + 1];
        v[2] = 2;
        for i in 3..n + 1 {
            v[i] = i;
            for j in list.iter() {
                if i % j == 0 {
                    v[i] = v[i].min(v[i / j] + j);
                    break;
                }
            }
        }
        return v[n] as i32;
    }
}
// @lc code=end
