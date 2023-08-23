/*
Given an array of positive integers nums and a positive integer target, return the minimal length of a
subarray
whose sum is greater than or equal to target. If there is no such subarray, return 0 instead.

Example 1:

Input: target = 7, nums = [2,3,1,2,4,3]
Output: 2
Explanation: The subarray [4,3] has the minimal length under the problem constraint.

Example 2:

Input: target = 4, nums = [1,4,4]
Output: 1

Example 3:

Input: target = 11, nums = [1,1,1,1,1,1,1,1]
Output: 0

*/

struct Solution;
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = 0;
        let mut sum = 0;
        let mut min = nums.len() as i32 + 1;

        while i < nums.len() {
            sum += nums[i];

            while sum > target {
                min = std::cmp::min(min, i as i32 - j as i32 + 1);

                if i == j {
                    break;
                }
                sum -= nums[j];
                j += 1;
            }

            if sum >= target {
                min = std::cmp::min(min, i as i32 - j as i32 + 1);
            }
            i += 1;
        }

        min % (nums.len() as i32 + 1)
    }
}

#[cfg(test)]
mod test {
    use crate::problems::min_subarray_sum::*;
    pub fn test_case(target: i32, nums: Vec<i32>, expected: i32) {
        println!("target: {} nums: {:?} expected: {}", target, nums, expected);
        assert_eq!(Solution::min_sub_array_len(target, nums), expected);
    }

    #[test]
    pub fn test_min_sub_array_len() {
        test_case(7, [2, 3, 1, 2, 4, 3].to_vec(), 2);
        test_case(4, [1, 4, 4].to_vec(), 1);
        test_case(11, [1, 1, 1, 1, 1, 1, 1, 1].to_vec(), 0);
        test_case(0, Vec::new(), 0);
        test_case(11, [1, 2, 3, 4, 5].to_vec(), 3);
    }
}
