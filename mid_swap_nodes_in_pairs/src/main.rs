// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}
impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        } 

        let mut node1 = head;
        let mut node2 = node1.as_mut().unwrap().next.take();
        
        node1.as_mut().unwrap().next = Solution::swap_pairs(node2.as_mut().unwrap().next.take());
        node2.as_mut().unwrap().next = node1;

        node2
    }
}

// algo explanation for [7,1,5,2]
//
// fn call 1:   7->1->5->2->none
// n1:          7->none                         (take() cuts off)
// n2:          1->5->2->none    |-> 1->none    (take() cuts off)
//                               |
// fn call 2:   5->2->none ------|
// n1:          5->none                         (take() cuts off)
// n2:          2->none          |-> 2->none    (take() cuts off)
//                               |
// fn call 3:   none ------------|
// return       none ----|
//                       |
// fn c2 cont:           |
// n1next:      none <---|
// n2next:      5->none
// return       2->5->none ----|  (n2 fnc 2 + n2next = 2->none + 5->none == 2->5->none)
//                             |
// fn c1 cont:                 |
// n1next:      2->5->none <---|
// n2next:      7->2->5->none     (n1 fnc 1 + n1next = 7->none + 2->5->none == 7->2->5->none)
// return       1->7->2->5->none  (n2 fnc 1 + n2next = 1->none + 7->2->5->none == ...)

// testing
struct Solution;
fn main() {
    assert_eq!(Solution::swap_pairs(arr_x_to_list(&[7,1,5,2])), arr_x_to_list(&[1,7,2,5]));
    assert_eq!(Solution::swap_pairs(arr_x_to_list(&[])), arr_x_to_list(&[]));
    assert_eq!(Solution::swap_pairs(arr_x_to_list(&[1,2,3])), arr_x_to_list(&[2,1,3]));
    assert_eq!(Solution::swap_pairs(arr_x_to_list(&[1,2,3,4,5,6,7,8,9])), arr_x_to_list(&[2,1,4,3,6,5,8,7,9]));
    assert_eq!(Solution::swap_pairs(arr_x_to_list(&[1])), arr_x_to_list(&[1]));
}

pub fn arr_x_to_list(array: &[i32]) -> Option<Box<ListNode>> {
    if array.len() == 0 {
        return None;
    }
    let len: usize = array.len()-1;
    let root: &mut ListNode = &mut ListNode::new(array[len]);
    
    for item in array[..len].iter().rev() {
        let h1: &mut ListNode = &mut ListNode::new(*item);
        h1.next = Some(Box::new(root.clone()));
        *root = h1.clone();
    }
    Some(Box::new(root.clone()))
}