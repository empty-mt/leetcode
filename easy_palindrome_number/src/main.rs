impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let palin: String = x.to_string();
        let reversed: String = palin.chars().rev().collect::<String>();
        return palin == reversed;
    }
}