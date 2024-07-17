

pub struct Solution;

impl Solution {
    fn find_longest_common_prefix(strs: Vec<String>) -> String {
        let result = "".to_string();

        result
    }
}

pub fn main() {
    let mut strs: Vec<String> = Vec::new(); 
    strs.push("flower".to_string());
    strs.push("flow".to_string());
    strs.push("flight".to_string());
    let prefix = Solution::find_longest_common_prefix(strs);
    println!("Longest common prefix is {}", prefix);
}
