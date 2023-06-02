/// https://leetcode.com/problems/two-sum/


impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for(index,data) in nums.iter().enumerate(){
                let complement = target - data ;
                if let Some(i) = map.get(&complement){
                      return [index as i32,*i as i32].into()
                }else{
                    map.insert(data,index as i32);
                }
        }
        [0,0].into()
    }
}
