/*
There is an integer array nums sorted in ascending order (with distinct values).

Prior to being passed to your function, nums is possibly rotated at an unknown pivot index k (1 <= k < nums.length) such that the resulting array is [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed). For example, [0,1,2,4,5,6,7] might be rotated at pivot index 3 and become [4,5,6,7,0,1,2].

Given the array nums after the possible rotation and an integer target, return the index of target if it is in nums, or -1 if it is not in nums.

You must write an algorithm with O(log n) runtime complexity.


Example 1:

Input: nums = [4,5,6,7,0,1,2], target = 0
Output: 4

Example 2:

Input: nums = [4,5,6,7,0,1,2], target = 3
Output: -1

Example 3:

Input: nums = [1], target = 0
Output: -1

*/

struct Solution;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut min = 0_usize;
        let mut max = nums.len() - 1;

        if max == min {
            if nums[0] == target {
                return 0;
            }
        }

        while max > min {
            let mid = (min + max) / 2;
            let mid_value = nums[mid];
            let left = nums[min];
            let right = nums[max];
            if left == target {
                return min as i32;
            }

            if right == target {
                return max as i32;
            }

            if mid_value == target {
                return mid as i32;
            }

            if left < mid_value {
                if target >= left && target <= mid_value {
                    max = mid;
                } else {
                    min = mid;
                }
            } else {
                if target >= mid_value && target <= right {
                    min = mid;
                } else {
                    max = mid;
                }
            }

            if max - min == 1 {
                break;
            }
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use crate::problems::search_in_rotated_array::*;
    pub fn test_case(target: i32, nums: Vec<i32>, expected: i32) {
        println!("target: {} nums: {:?} expected: {}", target, nums, expected);
        assert_eq!(Solution::search(nums, target), expected);
    }

    #[test]
    pub fn test_min_sub_array_len() {
        test_case(1, [1, 3].to_vec(), 0);
        test_case(8, [4, 5, 6, 7, 8, 1, 2, 3].to_vec(), 4);

        test_case(0, [4, 5, 6, 7, 0, 1, 2].to_vec(), 4);
        test_case(3, [4, 5, 6, 7, 0, 1, 2].to_vec(), -1);
        test_case(0, [1].to_vec(), -1);
        test_case(0, [0, 1, 2, 3, 4, 5, 6, 7].to_vec(), 0);
        test_case(0, [7, 6, 5, 4, 3, 2, 1, 0].to_vec(), 7);
        test_case(
            0,
            [5, 6, 7, 8, 9, 10, 11, 12, 13, 0, 1, 2, 3, 4].to_vec(),
            9,
        );
        test_case(-1, [-1, 0, 1, 2, 3, 4].to_vec(), 0);
    }
}
