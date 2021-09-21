/*
 * @lc app=leetcode id=365 lang=rust
 *
 * [365] Water and Jug Problem
 */

// @lc code=start
fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 || b == 0 {
        return 0;
    }
    let mut x = a;
    let mut y = b;
    while y != 0 {
        let z = x % y;
        x = y;
        y = z;
    }
    x
}

impl Solution {
    pub fn can_measure_water(
        jug1_capacity: i32,
        jug2_capacity: i32,
        mut target_capacity: i32,
    ) -> bool {
        if jug1_capacity + jug2_capacity < target_capacity {
            return false;
        }
        if jug1_capacity == 0 || jug2_capacity == 0 {
            return target_capacity == 0 || jug1_capacity + jug2_capacity == target_capacity;
        }
        target_capacity % gcd(jug1_capacity, jug2_capacity) == 0
    }
}
// @lc code=end
