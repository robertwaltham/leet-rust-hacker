/*

Given a string s, find the length of the longest
substring
without repeating characters.

Example 1:

Input: s = "abcabcbb"
Output: 3
Explanation: The answer is "abc", with the length of 3.

Example 2:

Input: s = "bbbbb"
Output: 1
Explanation: The answer is "b", with the length of 1.

Example 3:

Input: s = "pwwkew"
Output: 3
Explanation: The answer is "wke", with the length of 3.
Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.

Constraints:

    0 <= s.length <= 5 * 10^4
    s consists of English letters, digits, symbols and spaces.
*/

use std::cmp::max;
use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map = HashMap::new();
        let mut before = -1_i32;
        let mut longest = 0;

        for (current, c) in s.chars().enumerate() {
            if let Some(last) = map.insert(c, current as i32) {
                before = max(last, before);
            }
            longest = max(longest, current as i32 - before);
        }

        longest
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::longest_substring_without_repeat::*;
    fn test_case(s: &str, expected: i32) {
        println!("Test Case: {} Expected: {}", s, expected);
        assert_eq!(
            Solution::length_of_longest_substring(s.to_string()),
            expected
        );
    }

    #[test]
    fn test_length_of_longest_substring() {
        test_case("abcabcbb", 3);
        test_case("pwwkew", 3);
        test_case("bbbbb", 1);
        test_case("dvdf", 3);
    }
}
