/*
 * @lc app=leetcode id=121 lang=rust
 *
 * [121] Best Time to Buy and Sell Stock
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let len = prices.len();
        let mut max_profit = 0;
        let mut min_price = prices[0];
        for i in 1..len {
            let tmp = prices[i] - min_price;
            if tmp < 0 {
                min_price = prices[i];
            } else if tmp > max_profit {
                max_profit = tmp;
            }
        }
        if max_profit > 0 {
            max_profit
        } else {
            0
        }
    }
}
// @lc code=end
