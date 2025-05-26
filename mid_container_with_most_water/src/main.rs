// bruteforce solution

use std::cmp::{min, max};

impl Solution {
    pub fn max_area(height_vec: Vec<i32>) -> i32 {
        let mirror_vec = height_vec.clone();
        let mut vol_max: usize = 0;

        for (i, height) in height_vec.iter().enumerate() {
            for (j, m_height) in mirror_vec[i..].iter().enumerate() {
                let vol: usize =  *min(height, m_height) as usize * j;
                vol_max = max(vol_max, vol);
            }
        }

        vol_max as i32
    }
}

// testing
struct Solution;
fn main() {
    assert_eq!(Solution::max_area(vec![1,8,6,2,5,4,8,3,7]),49);
    assert_eq!(Solution::max_area(vec![2,2,2,2,2,2,2]),12);
    assert_eq!(Solution::max_area(vec![2,2,4,2,4,2,2]),12);
    assert_eq!(Solution::max_area(vec![2,2,4,2,4,1,1]),8);
    assert_eq!(Solution::max_area(vec![1,8,6,2,5,4,7,3,8]),56);
    assert_eq!(Solution::max_area(vec![1,1]),1);
}

