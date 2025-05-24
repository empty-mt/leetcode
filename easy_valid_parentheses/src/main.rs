impl Solution {
    pub fn can_be_closed (
            mut last: char, 
            stack: &mut std::collections::VecDeque::<char>, 
            option: char
        ) -> bool {
        // remember last item, if one was pushed before
        if !stack.is_empty() {
            last = *stack.back().unwrap();
        }
        // fitting parentheses found && item in stack
        if last == option && !stack.back().is_none() {
            stack.pop_back().unwrap();
            return true;
        } 
        // ..or not
        false
}

    pub fn is_valid(s: String) -> bool {
        let last: char = s.chars().nth(0).unwrap();
        let mut stack = std::collections::VecDeque::<char>::new();
        
        // not valid
        if s.len() % 2 != 0 {
            return false;
        } else {
            for item in s.chars() { 
                match item {
                '(' => stack.push_back(item),
                '[' => stack.push_back(item),
                '{' => stack.push_back(item),
                ')' => {
                        if !Self::can_be_closed(last, &mut stack, '('){
                            return false;
                        } 
                        ()
                    },
                ']' => {
                        if !Self::can_be_closed(last, &mut stack, '['){
                            return false;
                        }
                        ()
                    },
                '}' => { 
                        if !Self::can_be_closed(last, &mut stack, '{'){
                            return false;
                        }
                        ()
                    },
                _ => ()
                };
                //println!("item: {item} items pushed: {:?}", stack);
            }
            // valid
            if stack.is_empty() {
                return true;
            }
            false
        }
    }
}