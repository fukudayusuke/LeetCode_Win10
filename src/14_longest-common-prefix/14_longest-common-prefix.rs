/*
 * @lc app=leetcode id=14 lang=rust
 *
 * [14] Longest Common Prefix
 */

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
// @lc code=end

