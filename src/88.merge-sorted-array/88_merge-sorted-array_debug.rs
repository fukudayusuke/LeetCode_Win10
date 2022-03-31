/*
 * @lc app=leetcode id=88 lang=rust
 *
 * [88] Merge Sorted Array
 */

// @lc code=start
// use std::io;
// use std::str::FromStr;
// use sscanf::scanf;   // sscanf使いたいけどダメ
// use itertools::Itertools;

pub struct Solution;
use std::convert::TryInto;

impl Solution {
    pub fn merge(_nums1: &mut Vec<i32>, _m: i32, _nums2: &mut Vec<i32>, _n: i32) {
        let debug = _n as usize;
        let d2 = _nums1.len();
        _nums1.split_off(_nums1.len() - _n as usize);   // erase after n 
        _nums1.append(_nums2);                          // append
        _nums1.sort();                                  // sort
    }
}

fn main()   {
    let mut nums1 = vec![1,2,3,0,0,0];
    let m : i32 = 3;
    let mut nums2 = vec![2,5,6];
    let n : i32 = 3;

    
    Solution::merge( &mut nums1, m, &mut nums2, n );
    println!( "Leet Code 088" );
    println!( "we return {:?}", nums1 ); 
}

// @lc code=end

