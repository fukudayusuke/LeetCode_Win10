/*
 * @lc app=leetcode id=14 lang=rust
 *
 * [14] Longest Common Prefix
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String { 
        match strs.is_empty() {
            true => "".to_string(),
            _ => {
                strs.iter().skip(1).fold(strs[0].clone(), |acc, x| {
                     acc
                        .chars()
                        .zip(x.chars())
                        .take_while(|(x,y)| x == y)
                        .map(|(x, _)| x)
                        .collect()
                })
            }
        }
    }
}

fn main()   {                                                       // 
    let mut strings : Vec<String> = vec!["flower".to_string(),"flow".to_string(),"flight".to_string()];
    let mut answer : String = Solution::longest_common_prefix( strings );
    println!( "Leet Code 014" );
    println!( "we return [{one}]", one=answer );
}
// @lc code=end

