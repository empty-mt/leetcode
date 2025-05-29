use std::collections::HashSet;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut quads: Vec<Vec<i32>>;
        let mut nums_sorted: Vec<i32> = nums;
        let mut map: HashSet<Vec<i32>> = HashSet::<Vec<i32>>::new();

        // sort vec to walk in specific way:
        // if sum < 0 move left up, sum > 0 move right down
        nums_sorted.sort();
        
        for (j, num1) in nums_sorted.iter().enumerate() {
            // start left from j
            for (i, num2) in nums_sorted[j+1..].iter().enumerate() {
                // 4 pointer
                // ptr grows with j and i
                let mut ptr = nums_sorted[j+i+2..].iter();
                let mut ptr_l = ptr.next();
                let mut ptr_r = ptr.next_back();

                while let (Some(low), Some(high)) = (ptr_l, ptr_r) {
                    // check sum - i64 cause -10^9 <= nums[i] <= 10^9
                    let sum: i64 = (*num2 as i64 + *low as i64 + *high as i64 + *num1 as i64 ) as i64;
                    // println!("j: {:?}, i: {:?}, low: {:?}, ptr: {:?}, high: {:?} | sum: {:?}",num1, num2, low, nums_sorted[i+2], high, sum);
                    // found triplet: move ptr to may find other combinations with num
                    if sum == target as i64 {
                        // save in hashset to auto del doubles
                        let mut quad = vec![*num1, *num2, *low, *high];
                        quad.sort();
                        map.insert(quad);
                        ptr_l = ptr.next();
                        ptr_r = ptr.next_back();
                    // if sum < 0 incr left ptr 
                    } else if sum < target as i64 {
                        ptr_l = ptr.next();
                    // if sum > 0 decr right ptr
                    } else {
                        ptr_r = ptr.next_back();
                    }
                }
            }
        }
        // convert into vec[vec]
        // sort for testing
        quads = map.into_iter().collect();
        quads.sort();
        quads

    }
}

struct Solution;
fn main() {
    assert_eq!(Solution::four_sum(vec![1,0,-1,0,-2,2], 0), [[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]);
    assert_eq!(Solution::four_sum(vec![1,0,-1, 0], 0), [[-1,0,0, 1]]);
    assert_eq!(Solution::four_sum(vec![2,2,2,2,2], 8), [[2,2,2,2]]);
    assert_eq!(Solution::four_sum(vec![0,0,2,2,2,2,2,4,4], 6), [[0,0,2,4], [0,2,2,2]]);
    assert_eq!(Solution::four_sum(vec![0,0,0,2,2,2,2,2,4,4], 6), [[0,0,2,4], [0,2,2,2]]);
    assert_eq!(Solution::four_sum(vec![-3,-1,0,2,4,5], 1), [[-3,-1,0,5]]);
    assert_eq!(Solution::four_sum(vec![0,0,0,1000000000,1000000000,1000000000,1000000000], 1000000000), [[0,0,0,1000000000]]);
    assert_eq!(Solution::four_sum(vec![1000000000,1000000000,1000000000,1000000000], 0), Vec::<Vec<i32>>::new());
    assert_eq!(Solution::four_sum(vec![1000000000,1000000000,1000000000,1000000000], -294967296), Vec::<Vec<i32>>::new());
    assert_eq!(Solution::four_sum(vec![1000000000,1000000000,1000000000,1000000000], -294967296), Vec::<Vec<i32>>::new());
}
