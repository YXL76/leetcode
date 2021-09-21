/*
 * @lc app=leetcode id=8 lang=rust
 *
 * [8] String to Integer (atoi)
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut flag = false;
        let mut sign = 1;
        let s = s.trim_start();
        let max = vec![2, 1, 4, 7, 4, 8, 3, 6, 4, 7];
        let mut nums: Vec<i32> = Vec::with_capacity(s.len());

        let mut s = s.chars();
        let first = if let Some(first) = s.next() {
            first
        } else {
            return 0;
        };
        if first == '-' {
            sign = -1;
        } else if first != '+' {
            if let Some(num) = first.to_digit(10) {
                if flag || num > 0 {
                    flag = true;
                    nums.push(num as i32);
                }
            } else {
                return 0;
            }
        }

        for c in s {
            if let Some(num) = c.to_digit(10) {
                if flag || num > 0 {
                    flag = true;
                    nums.push(num as i32);
                }
            } else {
                break;
            }
        }

        if nums.len() > 10 {
            return if sign == 1 { i32::MAX } else { i32::MIN };
        }

        if nums.len() == 10 {
            for i in 0..10 {
                if nums[i] > max[i] {
                    return if sign == 1 { i32::MAX } else { i32::MIN };
                } else if nums[i] < max[i] {
                    break;
                }
            }
        }

        let mut b = 1;
        let mut ans = 0;
        for i in (0..nums.len()).rev() {
            ans += sign * nums[i] * b;
            b *= 10;
        }

        ans
    }
}
// @lc code=end
