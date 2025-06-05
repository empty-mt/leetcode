// has to be O(Log n)

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut middle: i32 = (nums.len()/2) as i32;
        let mut low: i32 = 0;
        let mut high: i32 = match (nums.len().wrapping_sub(1)) as i32 {
            a => a 
        };
        let mut i: usize = 0;
        let mut result: Vec<i32> = vec![-1, -1];

        // binary search from easy_seach_insert_position
        while high >= low && i < nums.len() {
            if target == nums[middle as usize] {
                // binary search until first target hit
                let l = nums.partition_point(|&x| x < target) as i32;
                // binary search until last target dup hit, -1 for getting pos of last target
                let h = nums.partition_point(|&x| x <= target) as i32 - 1;
                
                result.clear();
                result.push(l);
                result.push(h);
                return result;
            // shift search area right
            } else if target > nums[middle as usize] {
                low = middle + 1; 
            // shift search area left
            } else if target < nums[middle as usize] {
                high = middle - 1;
            }
            // no target -> new mid position 
            middle = ((high - low)/2)+low;
            i += 1;
        }
        result
    }
}

struct Solution;
fn main() {
    // nums = [], target = 0 ->  [-1, -1]
    // nums = [5], target = 5 -> [0 ,  0]
    assert_eq!(Solution::search_range(vec![], 0), vec![-1, -1]);
    assert_eq!(Solution::search_range(vec![1,2,2,2,2,3,4,5,5,5,5,6,7,8,9,10,11,12,12,12,12,12,13], 2), vec![1,4]);
    //                                     0,1,2,3,4,5 
    assert_eq!(Solution::search_range(vec![5,7,7,8,8,10], 8), vec![3, 4]);
    assert_eq!(Solution::search_range(vec![1,2,2,2,2,3,4,5,5,5,5,6,7,8,9,10,11,12,12,12,12,12,13], 10), vec![15,15]);
    assert_eq!(Solution::search_range(vec![5], 5),vec![0, 0]);
    assert_eq!(Solution::search_range(vec![5], 6), vec![-1, -1]);
    assert_eq!(Solution::search_range(vec![5,7,7,8,8,10], 6), vec![-1, -1]);
    assert_eq!(Solution::search_range(vec![7,8,8,8,8,8,8,8,8,8,8,9], 7), vec![0,0]);
    assert_eq!(Solution::search_range(vec![8,8,8,8,8,8,8,8,8,8], 8), vec![0,9]);
    assert_eq!(Solution::search_range(vec![-999985131,-999953607,-999953607,-999915742,-999883817,-999849817,-999822901,-999815377,-999810801,-68594,-49967,20394,114012,999969829,999973689,999975494]
    , -999953607), vec![1,2]);
}
