//struct Solution;
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut middle: i32 = (nums.len()/2) as i32;
        let mut low: i32 = 0;
        let mut high: i32 = (nums.len()-1) as i32;
        let mut i: usize = 0;
        
        while high >= low && i < nums.len() {
            //println!("i: [{i}] {low} [{:?}] < {middle} [{:?}] < {high} [{:?}]", nums[low as usize], nums[middle as usize], nums[high as usize]);
            if target == nums[middle as usize] {
                return middle;
            } else if target > nums[middle as usize] {
                // right
                low = middle + 1; // low can be > high
            } else if target < nums[middle as usize] {
                // left
                high = middle - 1; // low can be < high
            }
            middle = ((high - low)/2)+low;
            i += 1;
        }
        middle
    }
}
/*
// testing
fn main() {             
    assert_eq!(Solution::search_insert(vec![-3,0,1,3,5,6,7,8,9,12,15,90,100,111], 4), 4);
    assert_eq!(Solution::search_insert(vec![-3,0,1,3,5,6,7,8,9,12,15,90,100,111], 111), 13);
    assert_eq!(Solution::search_insert(vec![-3,0,1,3,5,6,7,8,9,12,15,90,100], -3), 0);
    assert_eq!(Solution::search_insert(vec![1,3,5,6], 5), 2);
    assert_eq!(Solution::search_insert(vec![1,3,5,6], 2), 1);
    assert_eq!(Solution::search_insert(vec![1,3,5,6], 7), 4);
    assert_eq!(Solution::search_insert(vec![1,3,5,6], 0), 0);
    assert_eq!(Solution::search_insert(vec![1,3,5,6], 1), 0);
}
*/