/*
You are given an array of integers nums, there is a sliding window of size k which is moving from the very left of the array to the very right. You can only see the k numbers in the window. Each time the sliding window moves right by one position.

Return the max sliding window.
*/

struct Solution;
use std::cmp::max;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut left_pass: Vec<i32> = vec![0; nums.len()];
        let mut right_pass: Vec<i32> = vec![0; nums.len()];
        let last = nums.len() - 1;
        left_pass[0] = nums[0];
        right_pass[last] = nums[last];

        for i in 1..nums.len() {
            if i % k as usize == 0 {
                left_pass[i] = nums[i];
            } else {
                left_pass[i] = max(left_pass[i - 1], nums[i]);
            }

            let j = nums.len() - i - 1;
            if (j as i32 + 1) % k == 0 {
                right_pass[j] = nums[j];
            } else {
                right_pass[j] = max(right_pass[j + 1], nums[j]);
            }
        }

        let mut result = Vec::with_capacity(nums.len() - k as usize);
        for i in 0..nums.len() - k as usize + 1 {
            result.push(max(right_pass[i], left_pass[i + k as usize - 1]));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::max_sliding_window::*;
    fn test_case(nums: Vec<i32>, k: i32, expected: Vec<i32>) {
        assert_eq!(
            // "{:?} expected: {:?}",
            Solution::max_sliding_window(nums, k),
            expected
        );
    }

    #[test]
    fn test_max_sliding_window() {
        test_case(
            [1, 3, -1, -3, 5, 3, 6, 7].to_vec(),
            3,
            [3, 3, 5, 5, 6, 7].to_vec(),
        );
        test_case(
            [2, 1, 3, 4, 6, 3, 8, 9, 10, 12, 56].to_vec(),
            4,
            [4, 6, 6, 8, 9, 10, 12, 56].to_vec(),
        );
    }
}
