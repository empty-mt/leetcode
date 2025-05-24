impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut temp: String = String::new();
        let mut res: String;
        let mut rem: String;
        let mut len: usize = usize::MAX;
        
        // get len smallest item
        for item in strs.iter() {
            if len > item.len() {
                len = item.len();
                temp = item.clone();
            } else if item.len() < 1 {
                return "".to_string();
            }
        }
        
        res = temp.clone();

        for item in strs.iter() {
            rem = String::new();
            // search until len smallest item
            for (i, char) in item[..len].chars().enumerate() {
                // compare item char with smallest item char
                if  temp.chars().nth(i) == Some(char) {
                    rem.push(char);
                // edge case: break if temp == item
                } else {
                    break;
                } 
            }
            // store if smaller sequence found
            if rem.len() < res.len() {
                res = rem.clone();
            }
        }
        res
    }
}