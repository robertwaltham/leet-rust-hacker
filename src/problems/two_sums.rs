/*

Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.

Example 1:

Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].

Example 2:

Input: nums = [3,2,4], target = 6
Output: [1,2]

Example 3:

Input: nums = [3,3], target = 6
Output: [0,1]

 */

pub fn test_two_sum() {
    assert!(two_sum([3, 3].to_vec(), 6) == [0, 1]);
    assert!(two_sum([2, 7, 11, 15].to_vec(), 9) == [0, 1]);
    assert!(two_sum([3, 2, 4].to_vec(), 6) == [1, 2]);
    println!("Finished testing two_sum")
}

use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut value_map: HashMap<i32, i32> = HashMap::new();

    for (i, first) in nums.iter().enumerate() {
        if value_map.contains_key(&(target - first)) {
            return [value_map[&(target - first)], i as i32].to_vec();
        } else {
            value_map.insert(*first, i as i32);
        }
    }

    return [-1, -1].to_vec();
}
