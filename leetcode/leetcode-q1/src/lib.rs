pub struct Solution;

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let concatenation = nums.iter().chain(nums.iter()).cloned().collect();

        concatenation
    }
    // ===== 여기에 LeetCode 터미널 코드 붙여넣기 =====
    // pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> { ... }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    // ===== LeetCode Examples 여기에 추가 =====
    // #[test] fn example_1() { assert_eq!(Solution::함수명(args), expected); }
}
