/*
 * @lc app=leetcode id=75 lang=rust
 *
 * [75] Sort Colors
 */

pub struct Solution;

// @lc code=start

fn qsort(nums: &mut Vec<i32>, left: usize, right: usize) {
    let mut i = left + 1;
    let mut j = right;
    let pivot = nums[left];
    while i <= j {
        while i <= right && nums[i] < pivot {
            i += 1;
        }
        while j >= left && nums[j] > pivot {
            j -= 1;
        }
        if i <= j {
            nums.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
    if left < j {
        nums.swap(left, j);
        qsort(nums, left, j);
    }
    if i < right {
        qsort(nums, i, right);
    }
}

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        qsort(nums, 0, nums.len() - 1);
    }
}
// @lc code=end
