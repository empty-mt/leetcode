use std::cmp::max;

struct Solution;

impl Solution {
    pub fn expand(str: & Vec<char>, len: isize, mut prev: isize, mut next: isize) -> isize {
        // expand from middle until !cond  
        while prev >= 0 && next < len &&str[prev as usize] == str[next as usize] {
            prev -= 1;
            next += 1;
        }
        next - prev - 1
    }
    pub fn longest_palindrome(s: String) -> String {
        let str: Vec<char> = s.chars().collect();
        let len: isize = s.len() as isize;
        let mut curr_len: isize;
        let mut max_len: isize = 1;
        let mut start_index: isize = 0;
        
        // case: s is always palin
        if s.len() <= 1 {
            return s;
        }

        for (i, _char) in s.chars().enumerate() {
            let odd_len: isize;
            let even_len: isize;
            // odd:  walk from same char
            odd_len = Self::expand(&str, len, i as isize, i as isize);
            // even: walk from char and next char
            even_len = Self::expand(&str, len, i as isize, i as isize + 1);
            // take bigger palin
            curr_len = max(even_len, odd_len);
            
            // update longest palin
            if curr_len > max_len {
                max_len = curr_len;
                // move start to i - actual_len/2
                start_index = i as isize - (curr_len - 1) / 2;
            }
        } 
        // build string
        s[start_index as usize..(start_index + max_len) as usize].to_string()
    }
}