
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut is_negative: bool  = false;
        let mut is_leading: bool  = false;
        let mut str: String = "".to_string();
        
        if s.is_empty() { return s.len() as i32;}
        
        // filter string
        for ch in s.chars() {
            match ch {
                ' ' => { 
                    if is_leading {
                        break;
                    }
                    continue;
                },
                '-' => {
                    if is_negative || is_leading {
                        break;
                    }
                    str.push(ch);
                    is_negative = true; 
                    is_leading = true;
                    continue;
                },
                '+' => {
                    if is_negative || is_leading {
                        break;
                    }
                    is_leading = true;
                    continue;
                },
                a => {
                    match a.to_digit(10) {
                        Some(_) => {
                            is_leading = true; 
                            str.push(a);
                        },
                        None => break
                    }
                },
            }
        }

        if str == "-" || str == "" { return 0; }

        // convert str, check overflow
        match str.parse::<i32>() {
            Ok(a) => a,
            Err(_) => {
                if is_negative {
                    return i32::MIN;
                }
                i32::MAX 
            }
        }
        
    }
}

// testing
struct Solution;
pub fn main() {
    assert_eq!(Solution::my_atoi("+0011".to_string()), 11);
    assert_eq!(Solution::my_atoi("+  0011".to_string()), 0);
    assert_eq!(Solution::my_atoi("-  0011".to_string()), 0);
    assert_eq!(Solution::my_atoi("-0011".to_string()), -11);
    assert_eq!(Solution::my_atoi("       -1111114242-+0a0".to_string()), -1111114242);
    assert_eq!(Solution::my_atoi("       +1111114242-+0a0".to_string()), 1111114242);
    assert_eq!(Solution::my_atoi("       1111114242-+0a0".to_string()), 1111114242);
    assert_eq!(Solution::my_atoi("+111a1114242-+0a0".to_string()), 111);
    assert_eq!(Solution::my_atoi("  -1 111114242-+0a0".to_string()), -1);
    assert_eq!(Solution::my_atoi("2-+0a0".to_string()), 2);
    assert_eq!(Solution::my_atoi("0a0".to_string()), 0);
    assert_eq!(Solution::my_atoi("        0a0".to_string()), 0);
    assert_eq!(Solution::my_atoi("-0a0".to_string()), 0);
    assert_eq!(Solution::my_atoi("".to_string()), 0);
    assert_eq!(Solution::my_atoi("1337c0d3".to_string()), 1337);
    assert_eq!(Solution::my_atoi("0-1".to_string()), 0);
    assert_eq!(Solution::my_atoi("-c".to_string()), 0);
    assert_eq!(Solution::my_atoi("c-".to_string()), 0);
    assert_eq!(Solution::my_atoi("-+".to_string()), 0);
    assert_eq!(Solution::my_atoi("+-".to_string()), 0);
    assert_eq!(Solution::my_atoi("++".to_string()), 0);
    assert_eq!(Solution::my_atoi("+-++1".to_string()), 0);
    assert_eq!(Solution::my_atoi("  c-".to_string()), 0);
    assert_eq!(Solution::my_atoi("  c2342334-".to_string()), 0);
    assert_eq!(Solution::my_atoi("  2-34c2334-".to_string()), 2);
    assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
    assert_eq!(Solution::my_atoi("+91283472332".to_string()), 2147483647);
    assert_eq!(Solution::my_atoi("91283472332".to_string()), 2147483647);
}


