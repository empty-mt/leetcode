struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        nums.len() as i32
    }
}

// testing

fn main() {
    let mut vector = vec![1,1,2,3,4];
    assert_eq!(Solution::remove_duplicates(&mut vector), 4);
    assert_eq!(Solution::remove_duplicates(&mut vec![0,0,1,1,1,2,2,3,3,4]), 5);
    assert_eq!(Solution::remove_duplicates(&mut vec![0,1,2,2,3,3,4]), 5);
    assert_eq!(Solution::remove_duplicates(&mut vec![-5,-2,-2,0,1,2,3,4]), 7);
    assert_eq!(Solution::remove_duplicates(&mut vec![0]), 1);
    assert_eq!(Solution::remove_duplicates(&mut vec![0,0,1,2,3,4]), 5);
}
