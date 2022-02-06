
pub struct Solution;
use std::collections::HashMap;

impl Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut compliments: HashMap<i32, i32> = HashMap::new();        //
        for i in 0..nums.len()  {                                       //
            let mut debug = nums[i];
            match compliments.get(&nums[i]) {                           //
                Some(&x) => return vec![x, i as i32],                   //
                None => compliments.insert(target - nums[i], i as i32), //
            };
        }
        return vec![-1,-1];                                             //
    }
}

fn main()   {                                                       // 
    // let mut nums = [2,7,11,15];
    // let mut nums : Vec<i32> = {2,7,11,15};
    let mut nums = vec![2,7,11,15];
    // let mut answer : Vec<i32> = two_sum( nums.to_vec(), 17 );                   //  並列をvec型cast?
    let mut answer : Vec<i32> = Solution::two_sum( nums, 17 );
    println!( "Leet Code 001" );
    println!( "we return [{one},{two}]", one=answer[0], two=answer[1] );
}