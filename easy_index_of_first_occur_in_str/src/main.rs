// struct Solution;
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let result: Option<usize> = haystack.find(needle.as_str());
        match result {
            Some(_) => {result.unwrap() as i32},
            None => {-1}
        }
    }
}
/*
// testing
fn main() {
    assert_eq!(Solution::str_str("sadbutsad".to_string(), "sad".to_string()), 0);
    assert_eq!(Solution::str_str("leetcode".to_string(), "leeto".to_string()), -1);
    assert_eq!(Solution::str_str("banana".to_string(), "ana".to_string()), 1);
    assert_eq!(Solution::str_str("banana".to_string(), "na".to_string()), 2);
    assert_eq!(Solution::str_str("jashnjashiqewjashjash".to_string(), "jashi".to_string()), 5);
}
*/