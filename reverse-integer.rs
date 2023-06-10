//https://leetcode.com/problems/reverse-integer

impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut ans = 0;
        use std::i32::{MAX as i32max ,MIN as i32min};
        while x != 0{
            if ans > i32max/10 || ans < i32min/10{
                return 0;
            }else{
                 ans*=10;
                 ans+=x%10;
                 x/=10;
            }

        } 
        ans
