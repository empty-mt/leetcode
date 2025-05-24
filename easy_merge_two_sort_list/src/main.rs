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

struct Solution;

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1 = list1;
        let mut l2 = list2;
        // if one list ends, return the other until both end
        if l2.is_none() {
            return l1;
        }
        if l1.is_none() {
            return l2;
        }
        // l1 val is lower (or eq) -> l1.next recursive with l1.next.ref and l2 -> ret modified l1
        if l1.as_ref().unwrap().val <= l2.as_ref().unwrap().val {
            l1.as_mut().unwrap().next = Self::merge_two_lists(l1.as_ref().unwrap().next.clone(), l2);
            return l1;
        // l2 val is lower -> l2.next recursive with li and l2.next.ref -> ret modified l2
        } else {
            l2.as_mut().unwrap().next = Self::merge_two_lists(l1, l2.as_ref().unwrap().next.clone()); 
            return l2;
        }
    }
}

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

// testing

fn main() {
    let node_list_1 = arr_x_to_list(&[-2,1,6,8]);
    let node_list_2 = arr_x_to_list(&[-20,-1,1,2,7]);
    let result = Solution::merge_two_lists(node_list_1, node_list_2);
    println!("{result:?}");
}
