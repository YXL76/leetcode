/*
 * @lc app=leetcode id=215 lang=rust
 *
 * [215] Kth Largest Element in an Array
 */

// @lc code=start
fn qsort(arr: &mut Vec<i32>, l: usize, r: usize) {
    let pivot = arr[l];
    let mut i = l + 1;
    let mut j = r;
    while i <= j {
        while i <= r && arr[i] < pivot {
            i += 1;
        }
        while j >= l && arr[j] > pivot {
            j -= 1;
        }
        if i <= j {
            arr.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
    if j > l {
        arr.swap(l, j);
        qsort(arr, l, j);
    }
    if i < r {
        qsort(arr, i, r);
    }
}

impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, mut k: i32) -> i32 {
        let len = nums.len();
        qsort(&mut nums, 0, len - 1);
        nums[len - k as usize]
    }
}
// @lc code=end
