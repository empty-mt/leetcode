// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

// struct Solution;
impl Solution {
    pub fn add_two_numbers_carry(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, mut carry: i32) -> Option<Box<ListNode>> {
                // match different len of lists
        match (l1, l2) {
            (Some(x), Some(y)) => {
                //println!("both:\n{x:?}\n{y:?}");
                // build sum for sum_node
                let mut sum: i32 = x.val + y.val + carry;
                // reset carry
                carry = 0;
                if sum >= 10 {
                    sum %= 10;
                    carry = 1;
                }
                // build new nodes - no cloning
                let node_1 = ListNode {val: x.val, next: x.next};
                let node_2 = ListNode {val: y.val, next: y.next};
                // rec with new nodes
                let node_sum = ListNode {val: sum, next: Self::add_two_numbers_carry(node_1.next, node_2.next, carry)};
                // return sum_node
                Some(Box::new(node_sum))
            },
            (None, Some(y)) => {
                let mut sum: i32 = y.val + carry;   
                carry = 0;
                if sum >= 10 {
                    sum %= 10;
                    carry = 1;
                }
                let node_2 = ListNode {val: y.val ,next: y.next};
                let node_sum = ListNode {val: sum, next: Self::add_two_numbers_carry(None, node_2.next, carry)};
                Some(Box::new(node_sum))
            },
            (Some(x), None) => {
                let mut sum: i32 = x.val + carry;
                carry = 0;
                if sum >= 10 {
                    sum %= 10;
                    carry = 1;
                }
                let node_1 = ListNode {val: x.val ,next: x.next};
                let node_sum = ListNode {val: sum, next: Self::add_two_numbers_carry(node_1.next, None, carry)};
                Some(Box::new(node_sum))
            },
            (None, None) => {
                if carry != 0 {
                    let node_sum = ListNode {val: 1, next: None};
                    return Some(Box::new(node_sum));
                }
                None
                
            },
        }
    }
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let carry: i32 = 0;
        Self::add_two_numbers_carry(l1, l2, carry)
    }
}


// testing
/*
fn main() {
    let node_list_1 = arr_x_to_list(&[1,1,6,8]);
    let node_list_2 = arr_x_to_list(  &[1,2,7]);
    
    assert_eq!(
        Solution::add_two_numbers(
            arr_x_to_list(&[9,9,9,9,9,9,9]), 
            arr_x_to_list(&[9,9,9,9])), 
        arr_x_to_list(&[8,9,9,9,0,0,0,1])
    );
    assert_eq!(
        Solution::add_two_numbers(
            arr_x_to_list(&[0]), 
            arr_x_to_list(&[0])), 
        arr_x_to_list(&[0])
    );
    assert_eq!(
        Solution::add_two_numbers(
            arr_x_to_list(&[2,4,5]), 
            arr_x_to_list(&[5,6,4])), 
        arr_x_to_list(&[7,0,0,1])
    );
    assert_eq!(
        Solution::add_two_numbers(
            arr_x_to_list(&[2,4,3]), 
            arr_x_to_list(&[5,6,4])), 
        arr_x_to_list(&[7,0,8])
    );
    assert_eq!(
        Solution::add_two_numbers(
            node_list_1, 
            node_list_2), 
        arr_x_to_list(&[2,3,3,9])
    );
}

// var arr size param
pub fn arr_x_to_list(array: &[i32]) -> Option<Box<ListNode>> {
    if array.len() == 0 {
        return None;
    }
    let len: usize = array.len()-1;
    let root: &mut ListNode = &mut ListNode::new(array[len]);
    
    for item in array[..len].iter().rev() {
        let temp: &mut ListNode = &mut ListNode::new(*item);
        temp.next = Some(Box::new(root.clone()));
        *root = temp.clone();
    }
    Some(Box::new(root.clone()))
}
*/