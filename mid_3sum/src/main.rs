use std::collections::HashSet;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut triplets: Vec<Vec<i32>>;
        let mut nums_sorted: Vec<i32> = nums;
        let mut map: HashSet<Vec<i32>> = HashSet::<Vec<i32>>::new();

        // sort vec to walk in specific way:
        // if sum < 0 move left up, sum > 0 move right down
        nums_sorted.sort();
        
        for (i, num) in nums_sorted.iter().enumerate() {
            // 3 pointer
            // start, left, right
            let mut ptr = nums_sorted[i+1..].iter();
            let mut ptr_l = ptr.next();
            let mut ptr_r = ptr.next_back();

            while let (Some(low), Some(high)) = (ptr_l, ptr_r) {
                // check sum
                let sum = num + low + high;
                // found triplet: move ptr to may find other combinations with num
                if sum == 0 {
                    // save in hashset to auto del doubles
                    map.insert(vec![*num, *low, *high]);
                    ptr_l = ptr.next();
                    ptr_r = ptr.next_back();
                // if sum < 0 incr left ptr 
                } else if sum < 0 {
                    ptr_l = ptr.next();
                // if sum > 0 decr right ptr
                } else {
                    ptr_r = ptr.next_back();
                }
            }
        }
        println!("result: {:?}", map);
        // convert into vec[vec]
        // sort for testing
        triplets = map.into_iter().collect();
        triplets.sort();
        triplets

    }
}

struct Solution;
fn main() {
    let x: Vec<Vec<i32>> = vec![];
    let mut y = vec![vec![1,0,-1]];
    y[0].sort();

    assert_eq!(Solution::three_sum(vec![-1,0,1,2,-1,-4]), vec![vec![-1,-1,2],vec![-1,0,1]]);
    assert_eq!(Solution::three_sum(vec![1,0,-1]), y);
    assert_eq!(Solution::three_sum(vec![1,0,1]), x);
    assert_eq!(Solution::three_sum(vec![-1,0,1,0]), vec![vec![-1,0,1]]);
    assert_eq!(Solution::three_sum(vec![0,0,0]), vec![vec![0,0,0]]);
    assert_eq!(Solution::three_sum(vec![-4,-2,-2,-2,0,1,2,2,2,3,3,4,4,6,6]), vec![[-4,-2,6],[-4,0,4],[-4,1,3],[-4,2,2],[-2, -2, 4],[-2,0,2]]);
}
