pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result: i32 = 0;
        let chars: Vec<char> = s.chars().collect(); // Vector to iterate over all chars
        let mut numbers: Vec<i32> = Vec::new(); // Vector to store the numeral values
        for element in chars {
            match element {
                'I' => numbers.push(1),     // can be placed before V and X
                'V' => numbers.push(5),
                'X' => numbers.push(10),    // can be placed before V and X
                'L' => numbers.push(50),
                'C' => numbers.push(100),   // can be placed before V and X
                'D' => numbers.push(500),
                'M' => numbers.push(1000),
                _ => {} // ignore every other possible char
            }
        }
        result
    }
}

fn main() {
    let s = "LVIII".to_string();
    let result = Solution::roman_to_int(s);
    println!("Your number is: {}", result);
}
