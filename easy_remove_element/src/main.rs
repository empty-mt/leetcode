//struct Solution;
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|x| *x != val);
        nums.len() as i32
    }
}

/*
// testing
fn main() {
    let mut vector = vec![1,1,2,3,4];
    assert_eq!(Solution::remove_element(&mut vector, 1), 3);
    assert_eq!(Solution::remove_element(&mut vec![0,0,1,1,1,2,2,3,3,4], 1), 7);
    assert_eq!(Solution::remove_element(&mut vec![0,1,2,2,3,3,4], 4), 6);
    assert_eq!(Solution::remove_element(&mut vec![-5,-2,-2,0,1,2,3,4], 0), 7);
    assert_eq!(Solution::remove_element(&mut vec![], 5), 0);
    assert_eq!(Solution::remove_element(&mut vec![0,0,1,2,3,4], 0), 4);
    assert_eq!(Solution::remove_element(&mut vec![0,1,2,2,3,0,4,2], 2), 5);
    assert_eq!(Solution::remove_element(&mut vec![3,2,2,3], 3), 2);
}
*/