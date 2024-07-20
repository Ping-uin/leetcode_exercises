pub struct Solution;

impl Solution {
    fn find_longest_common_prefix(strs: Vec<String>) -> String {
        let mut result = strs[0].clone();

        for char in &strs[1..] {
            while !char.starts_with(&result) {
                if result.is_empty() {
                    return "".to_string();
                }
                result.pop();
            }
        }
        result
    }
}

pub fn main() {
    let mut strs: Vec<String> = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string()
    ];
    let mut prefix = Solution::find_longest_common_prefix(strs);
    println!("First longest common prefix is {}", prefix);
    strs = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
    prefix = Solution::find_longest_common_prefix(strs);
    println!("Second longest common prefix is {}", prefix);
}
