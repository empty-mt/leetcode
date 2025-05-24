use std::collections::HashMap;
use std::cmp::max;
// struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut front = 0;
        let mut longest: i32 = 0;
        let mut temp: i32;
        let mut map: HashMap<char, i32> = HashMap::new();
        
        // empty 
        if s.len() == 0 {
            return longest;
        }
        // .. or 1 len str
        if s.len() == 1 {
            return longest + 1;
        }
        // i == back, which always moves
        for (i, item) in s.chars().enumerate() {
            if map.contains_key(&(item)) {
                temp = match map.get(&item) {
                    Some(x) => *x,
                    None => 0
                };
                // move "front" to position after last occurence of "temp" 
                if temp >= front {
                    front = temp + 1;
                }
                // else: do nothing, cause we are not in range[front, i] anymore
            }
            // fill map
            map.insert(item, i as i32);
            
            // is longer substr?
            longest = max(longest, i as i32 - front + 1);
        }
        longest
    }
}
/*
// testing
fn main() {
    assert_eq!(Solution::length_of_longest_substring("abcbadbd".to_string()), 4);
    assert_eq!(Solution::length_of_longest_substring("abc".to_string()), 3);
    assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
    assert_eq!(Solution::length_of_longest_substring("bbbbb".to_string()), 1);
    assert_eq!(Solution::length_of_longest_substring("jbpnbwwd".to_string()), 4);
    assert_eq!(Solution::length_of_longest_substring("bwwdjbpwn".to_string()), 6);
    assert_eq!(Solution::length_of_longest_substring("anviaj".to_string()), 5);
    assert_eq!(Solution::length_of_longest_substring("  a".to_string()), 2);
    assert_eq!(Solution::length_of_longest_substring("dvdf".to_string()), 3);
    assert_eq!(Solution::length_of_longest_substring("".to_string()), 0);
    assert_eq!(Solution::length_of_longest_substring("/".to_string()), 1);
    assert_eq!(Solution::length_of_longest_substring("abcdef".to_string()), 6);
    assert_eq!(Solution::length_of_longest_substring("!@#$%^&*()".to_string()), 10);
    assert_eq!(Solution::length_of_longest_substring("AaBbCcDd".to_string()), 8);
    assert_eq!(Solution::length_of_longest_substring("abcefcefxwab".to_string()), 7);
    assert_eq!(Solution::length_of_longest_substring(
        "abababababababababababababababababababababababababababababababab".to_string()), 
        2
    );
    assert_eq!(Solution::length_of_longest_substring("aaaaaaaabbbbbbbbcccccccc".to_string()), 2);
    assert_eq!(Solution::length_of_longest_substring("a b c a b c".to_string()), 3);
}
*/