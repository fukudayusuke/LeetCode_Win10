/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 */

// @lc code=start
pub struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for i in s.chars() {
            match i {
                '{' => stack.push('}'),                                     // case相当
                '(' => stack.push(')'),                                     // case相当
                '[' => stack.push(']'),                                     // case相当
                '}'|')'|']' if Some(i) != stack.pop() => return false,
                _ => (),                                                    // defalut相当
            }
            println!("i is {}", i);
        }   
        return stack.is_empty();
    }
}

fn main()   {                                                       // 
    let string : &str = "( )";
    let answer : bool = Solution::is_valid(string.to_string());
    println!("Leet Code 020");
    println!("we return {}", answer);
}
// @lc code=end

