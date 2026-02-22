pub struct Solution;

impl Solution {
    fn ft_atoi(s: &[char]) -> i32 {
        let mut result = 0;
        for &c in s.iter() {
            result = result * 26 + (c as u32 - 'A' as u32 + 1) as i32;
        }
        result
    }

    pub fn title_to_number(column_title: String) -> i32 {
        Self::ft_atoi(&column_title.chars().collect::<Vec<char>>())
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
