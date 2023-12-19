pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        return vec![];
    }
}
fn main() {
    println!("Two sum");
}

#[cfg(test)]

#[test]
fn test_case_1() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
}
#[test]
fn test_case_2() {
    let nums = vec![3, 2, 4];
    let target = 6;
    assert_eq!(Solution::two_sum(nums, target), vec![1, 2]);
}
#[test]
fn test_case_3() {
    let nums = vec![3, 3];
    let target = 6;
    assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
}
