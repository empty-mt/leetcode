use std::collections::VecDeque;

struct Solution;
impl Solution {
    pub fn concat(vec: &VecDeque<char>) -> i32 {
        /*   

        // use try_fold()

        vec.iter().fold(
            0, 
            |acc, elem|  {
                match (acc.checked_add(elem.to_digit(10).unwrap()), acc.checked_mul(10)) {
                    (Some(_a), Some(_b)) =>  {println!("00 {acc}"); acc*10 + elem.to_digit(10).unwrap()},
                    (Some(_a), None)         => {println!("x0 {acc}"); 0},
                    (None, Some(_a))         => {println!("0x {acc}"); 0},
                    (None, None)                => {println!("xx {acc}"); 0},
                }
            }
        )
        */
        
        let mut acc: i32 = 0;
        for elem in vec {
            match (acc.checked_add(elem.to_digit(10).unwrap() as i32) , acc.checked_mul(10)) {
                (Some(a), Some(_b)) =>  a,
                (Some(_a), None)    => {acc = 0; break},
                (None, Some(_a))    => {acc = 0; break},
                (None, None)    => {acc = 0; break},
                
            };
            acc *= 10;
            acc += elem.to_digit(10).unwrap() as i32;
        }
        acc
    }
    pub fn reverse(x: i32) -> i32 {
        let mut cnt: i32 = 0;
        let mut is_negative: bool = false;
        let mut result: i32;

        if x < 10 && x >= 0 {
            return x;
        }
        // int -> rev char vec
        let mut num_vec: VecDeque<char> = x
            .to_string()
            .chars()
            .rev()
            .map(|ch| ch)
            .collect();

        // cut all front 0
        loop {
            if num_vec.get(cnt as usize) == Some(&'0') {
                cnt += 1;
            } else { 
                break; 
            }
        }
        for _ in 0..cnt {
            num_vec.pop_front();
        }
        
        // check if last '-' and rm
        // num_vec.retain(|ch| *ch != '-'); 
        if num_vec.back() == Some(&'-') {
            is_negative = true;
            num_vec.pop_back();
        }
        
        // char vec -> int
        result = Self::concat(&num_vec) as i32;
        if is_negative {
            result = i32::from(result) * -1; 
        }

        result
    }
}

fn main() {
    assert_eq!(Solution::reverse(-2_147_483_648), 0);
    assert_eq!(Solution::reverse(2_147_483_647), 0);
    assert_eq!(Solution::reverse(-120001000), -100021);
    assert_eq!(Solution::reverse(-123), -321);
    assert_eq!(Solution::reverse(120), 21);
    assert_eq!(Solution::reverse(11), 11);
    assert_eq!(Solution::reverse(0), 0);
    assert_eq!(Solution::reverse(5), 5);
    assert_eq!(Solution::reverse(-5), -5);
    assert_eq!(Solution::reverse(123), 321);
    assert_eq!(Solution::reverse(-120030090), -90030021);
    assert_eq!(Solution::reverse(-2147483412), -2143847412);
    assert_eq!(Solution::reverse(1563847412), 0);
    assert_eq!(Solution::reverse(1534236469), 0);
    assert_eq!(Solution::reverse(-1563847412), 0);
}
