pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let input = x.to_string();
        let rev_input:String = input.chars().rev().collect();
        if input == rev_input {
            return true
        }
        false
    }
}

fn main(){
    println!("Palindrome Number");
}