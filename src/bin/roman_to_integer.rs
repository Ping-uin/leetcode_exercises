use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        // Assign each possible char its numeral value
        let mut roman_values: HashMap<char, i32> = HashMap::new();
        roman_values.insert('I', 1); // can be placed before V and X
        roman_values.insert('V', 5);
        roman_values.insert('X', 10); // can be placed before L and C
        roman_values.insert('L', 50);
        roman_values.insert('C', 100); // can be placed before D and M
        roman_values.insert('D', 500);
        roman_values.insert('M', 1000);

        let mut result = 0;
        let mut index = 0;
        let chars: Vec<char> = s.chars().collect(); // Extract all chars from the input string

        // Iterate over each char in string s
        while index < s.len() {
            let mut curr_num = roman_values.get(&chars[index]).unwrap();
            let mut next_num = if index < s.len() - 1 {
                roman_values.get(&chars[index + 1]).unwrap()
            } else {
                &0 // that works??
            };
            if curr_num < next_num {
                result += next_num - curr_num;
                index += 2;
            } else {
                result += curr_num;
                index += 1;
            }
        }
        result
    }
}

fn main() {
    let s = "MCMXCIV".to_string();
    let result = Solution::roman_to_int(s);
    println!("Your number is: {}", result);
}
