/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

// @lc code=start
pub struct Solution;
fn main()   {
    let  num : &str = "1213123";
    let  answer = Solution::is_palindrome(num);
    println!("{:?}", answer);
}
impl Solution {
    pub fn is_palindrome(x: &str) -> bool {
        let debug1 = x.to_string();
        let debug2 = debug1.chars();
        let debug3 = debug2.rev();
        let debug4 = debug3.collect::<String>();
        println!("{:?}", debug4);
        x.to_string()==x.to_string().chars().rev().collect::<String>()
    }
}
// @lc code=end

