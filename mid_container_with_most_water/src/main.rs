
use std::cmp::{min, max};

impl Solution {
    // bruteforce solution
    /* 
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
    */
    // pointer solution
    pub fn max_area(height_vec: Vec<i32>) -> i32 {
        let mut vol_max: i32 = 0;
        let mut left: usize = 0;
        let mut right: usize = height_vec.len() - 1;

        while left < right {
            let min_height: i32 = min(height_vec[left], height_vec[right]);
            let vol: i32 =  min_height * (right - left) as i32;

            vol_max = max(vol_max, vol);
            if min_height == height_vec[left] {
                left += 1
            } else {
                right -= 1
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

