/*
 * @lc app=leetcode id=88 lang=rust
 *
 * [88] Merge Sorted Array
 */

// @lc code=start
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        nums1.split_off(nums1.len() - n as usize);
        nums1.append(nums2);                       
        nums1.sort();                               
    }
}
// @lc code=end

