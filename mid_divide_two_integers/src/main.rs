
/* 
    assert_eq!(Solution::divide(i32::MIN, 1), i32::MIN) 
    ^ would be slow as f if we substract divisor once per loop.
    so we need an O(log(n)) approach using bit shift
*/

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let mut result: i32 = 0;
        let is_negative: bool = (divisor < 0) ^ (dividend < 0);
        // case: dsor/dend are i32::MIN ->  convert safely, Some(i32).abs() will cause conversion overflow
        let dsor: u32 = divisor.wrapping_abs() as u32;
        let mut dend: u32 = dividend.wrapping_abs() as u32;

        // case: ugly overflow testcase -> i32::MIN / -1 == i32::MAX + 1 
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }
        // case: simple calculation -> save runtime  
        if divisor == 1 {
            return dividend;
        }
        if divisor == -1 {
            return -dividend;
        }
        if dividend == divisor {
            return 1;
        }
        while dend >= dsor {
            let mut shift_cnt = 0;
            
            // ### 1st loop ###
            //
            // e.g. input: dend/dsor == 1000/2
            // double dsor until dsor > dend 
            // 1000 > (512 == 2 << (8+1)) -> shift_cnt == 8
            while dend > (dsor << (shift_cnt + 1)) {
                shift_cnt += 1;
            }
            // (result += 1 << 8) == (0 + 256)
            result += 1 << shift_cnt;
            // (dend -= 2 << 9) == (1000 - 512) == 488
            dend -= dsor << shift_cnt;
            // ### 2nd loop ###
            //
            // input: 488/2
            // ...
        }
        // case: is negative
        match is_negative {
            true => -result,
            false => result
        }
    }
    
}

struct Solution;
fn main() {
    assert_eq!(Solution::divide(10, 3), 3);
    assert_eq!(Solution::divide(10, -3), -3);
    assert_eq!(Solution::divide(7, -2), -3);
    assert_eq!(Solution::divide(-7, -2), 3);
    assert_eq!(Solution::divide(-7, 2), -3);
    assert_eq!(Solution::divide(i32::MAX, i32::MAX), 1);
    assert_eq!(Solution::divide(1981445587, 1172010393), 1);
    assert_eq!(Solution::divide(1100540749, -1090366779), -1);
    assert_eq!(Solution::divide(i32::MAX, 2), (i32::MAX/2));
    assert_eq!(Solution::divide(i32::MIN, i32::MAX), -1);
    assert_eq!(Solution::divide(i32::MIN, 1), i32::MIN);
    assert_eq!(Solution::divide(i32::MIN, i32::MIN), 1);
    assert_eq!(Solution::divide(i32::MIN, -1), i32::MAX);
    assert_eq!(Solution::divide(-1, i32::MIN), 0);
    assert_eq!(Solution::divide(1, i32::MIN), 0);
}
