/*
 * @lc app=leetcode id=4 lang=rust
 *
 * [4] Median of Two Sorted Arrays
 */

// @lc code=start
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let m = nums1.len();
        let n = nums2.len();
        let middle = (m + n) / 2;
        let mut i = 0;
        let mut j = 0;
        let mut prev = 0;
        let mut curr = 0;

        while i + j <= middle && i < m && j < n {
            prev = curr;
            if nums1[i] < nums2[j] {
                curr = nums1[i];
                i += 1;
            } else {
                curr = nums2[j];
                j += 1;
            }
        }

        if i + j <= middle {
            if i == m {
                if i + j == middle {
                    prev = curr;
                    curr = nums2[j];
                } else {
                    j = middle - i;
                    prev = nums2[j - 1];
                    curr = nums2[j];
                }
            } else if j == n {
                if i + j == middle {
                    prev = curr;
                    curr = nums1[i];
                } else {
                    i = middle - j;
                    prev = nums1[i - 1];
                    curr = nums1[i];
                }
            }
        }

        if (m + n) % 2 == 0 {
            (prev + curr) as f64 / 2.0
        } else {
            curr as f64
        }
    }
}
// @lc code=end
