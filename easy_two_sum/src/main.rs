use std::{collections, vec};

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = collections::HashMap::new();
        let mut result = vec![];
        
        for (i, val) in nums.iter().enumerate() {
            let diff = target - val;
            let expected = map.contains_key(&diff);
            if expected {
                result.push(i as i32);
                result.push(*(map.get(&diff).unwrap()) as i32);
            }
            map.insert(val, i);
        }
        result
    }
}