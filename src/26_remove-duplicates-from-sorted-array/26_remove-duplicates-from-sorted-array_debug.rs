/*
 * @lc app=leetcode id=26 lang=rust
 *
 * [26] Remove Duplicates from Sorted Array
 */

// @lc code=start
pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        match nums.is_empty() {
            true => 0,
            false => {
                let mut prev = 0;
                for i in 1..nums.len() {
                    if nums[prev] != nums[i] {
                        prev += 1;
                        nums[prev] = nums[i];
                    }
                }
                (prev + 1) as i32
            }
        }
    }
}

fn main()   {
    let nums : Vec<i32> = vec![0,0,1,1,1,2,2,3,3,4];
    // let answer : <i32> = Solution::remove_duplicates(&nums);
    Solution::remove_duplicates(nums: &mut Vec<i32>)(&nums);
    println!("leetcode026");
    // println!("answer {}", nums)
}
// @lc code=end

