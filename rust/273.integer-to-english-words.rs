/*
 * @lc app=leetcode id=273 lang=rust
 *
 * [273] Integer to English Words
 */

// @lc code=start
impl Solution {
    pub fn number_to_words(mut num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }

        const table0: [&str; 3] = ["Thousand", "Million", "Billion"];

        const table1: [&str; 19] = [
            "One",
            "Two",
            "Three",
            "Four",
            "Five",
            "Six",
            "Seven",
            "Eight",
            "Nine",
            "Ten",
            "Eleven",
            "Twelve",
            "Thirteen",
            "Fourteen",
            "Fifteen",
            "Sixteen",
            "Seventeen",
            "Eighteen",
            "Nineteen",
        ];

        const table2: [&str; 8] = [
            "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
        ];

        let mut ans = Vec::new();
        let mut i = -2;
        while num > 0 {
            i += 1;
            let (h, s) = {
                let now = num % 1000;
                num /= 1000;
                if now == 0 {
                    continue;
                }
                (now / 100, now % 100)
            };
            if i >= 0 {
                ans.push(table0[i as usize]);
            }
            if s > 0 {
                if s < 20 {
                    ans.push(table1[(s - 1) as usize]);
                } else {
                    let g = s % 10;
                    if g > 0 {
                        ans.push(table1[(g - 1) as usize]);
                    }
                    ans.push(table2[(s / 10 - 2) as usize]);
                }
            }
            if h > 0 {
                ans.push("Hundred");
                ans.push(table1[(h - 1) as usize]);
            }
        }

        ans.reverse();
        ans.join(" ")
    }
}
// @lc code=end
