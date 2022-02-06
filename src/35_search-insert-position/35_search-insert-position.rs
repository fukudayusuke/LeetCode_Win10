/*
 * @lc app=leetcode id=35 lang=rust
 *
 * [35] Search Insert Position
 */

// @lc code=start
pub struct Solution;
fn main()   {
    let  answer = Solution::search_insert(vec![2,4,6,8,10],8 );
    println!("{:?}", answer);
}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let debug1 = nums.binary_search(&target);
        let debug2 = debug1.unwrap_or_else(|x| x) as i32;
        println!("{:?}", debug2);
        nums.binary_search(&target).unwrap_or_else(|x| x) as i32
    }
}
// @lc code=end

