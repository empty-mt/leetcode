use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut sum: i32 = 0;
        let mut subsequence: VecDeque<char> = Default::default();
        let dict = HashMap::<char, i32>::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);
        
        for char in s.chars() {
            let mut add: i32;
            
            subsequence.push_back(char);
            add = *(dict.get(&char).unwrap());
            sum += add;
            
            if subsequence.len() > 1 {
                let first = subsequence.get(0).unwrap();
                let second = subsequence.get(1).unwrap();
                let mut merged = first.to_string();
                merged.push(*second);
                
                add = match merged.as_ref() {
                "CM" => 900,
                "CD" => 400,
                "XC" => 90,
                "XL" => 40,
                "IX" => 9,
                "IV" => 4,
                    _ => 0
                };
                if add != 0 {
                    sum += add - *(dict.get(&first).unwrap()) - *(dict.get(&char).unwrap());
                }
                subsequence.pop_front();
            } 
        }
        sum
    }
}