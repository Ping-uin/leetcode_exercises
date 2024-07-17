pub struct Solution;

impl Solution {
    fn find_longest_common_prefix(strs: Vec<String>) -> String {
        let mut result = "".to_string();
        let comparator = &strs[0].chars().collect::<Vec<_>>();
        let mut i = 0;
        let mut j = 0;
        while i < strs.len() {
            for char in strs[i].chars() {
                if char == comparator[j] {
                    result.push(char);
                    if j < strs.len() - 1 {
                        j += 1;
                    }
                } else {
                    break;
                }
            }
            i += 1;
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
    println!("First longest common prefix is {}", prefix);
}
