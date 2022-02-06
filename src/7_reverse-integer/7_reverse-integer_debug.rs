/*
 * @lc app=leetcode id=7 lang=rust
 *
 * [7] Reverse Integer
 */

// @lc code=start
pub struct Solution;

fn main() {                                                       // 
    let mut res = 001234567;
    res = Solution::reverse(res);
    println!("{}", res);
}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        fn helper(mut n: i32) -> Option<i32> {  // 無名関数挟んでリターンを合わせてる
            let mut res = 0i32;
            let mut  debug:i32;
            let mut  debug1;
            let mut  debug2: i32 = 0;
            while n.abs() != 0 {                //桁終わるまで
                debug = n.abs();        
                debug1 = res.checked_mul(10)?;
                debug2 +=  (n % 10); // 10で割った余りは一番最後の桁
                res = res.checked_mul(10)?.checked_add(n % 10)?;        //
                n /= 10;    //10割りして次の桁へ移動
            }
            Some(res)
        }
        helper(x).unwrap_or_default()
    }
}
// @lc code=end

