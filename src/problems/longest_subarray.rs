/*

Given a binary array nums, you should delete one element from it.

Return the size of the longest non-empty subarray containing only 1's in the resulting array. Return 0 if there is no such subarray.


Example 1:

Input: nums = [1,1,0,1]
Output: 3
Explanation: After deleting the number in position 2, [1,1,1] contains 3 numbers with value of 1's.

Example 2:

Input: nums = [0,1,1,1,0,1,1,0,1]
Output: 5
Explanation: After deleting the number in position 4, [0,1,1,1,1,1,0,1] longest subarray with value of 1's is [1,1,1,1,1].

Example 3:

Input: nums = [1,1,1]
Output: 2
Explanation: You must delete one element.


Constraints:

    1 <= nums.length <= 10^5
    nums[i] is either 0 or 1.


 */

pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }

    #[derive(Debug)]
    enum State {
        Start,
        Zero,
        One,
    }

    fn longest_value(longest: &mut i32, ptr: &mut usize, indicies: [[i32; 2]; 2]) {
        let current_start = indicies[*ptr][0];
        let current_end = indicies[*ptr][1];

        if current_start < 0 || current_end < 0 {
            return;
        }
        let current_length = current_end - current_start + 1;

        if current_length > *longest {
            *longest = current_length;
        }

        *ptr = (*ptr + 1) % 2;

        let last_end = indicies[*ptr][1];
        let last_start = indicies[*ptr][0];
        let last_length = last_end - last_start + 1;

        if last_end > -1 && (current_start - last_end).abs() < 3 {
            if last_length + current_length > *longest {
                *longest = last_length + current_length;
            }
        }
    }

    let mut indicies = [[-1_i32, -1_i32], [-1_i32, -1_i32]];
    let mut ptr = 0;
    let mut longest = 0_i32;
    let mut state = State::Start;
    let mut seen_zero = false;

    for i in 0..nums.len() {
        let value = nums[i];

        match state {
            State::Start => {
                if value == 1 {
                    indicies[ptr][0] = 0;
                    indicies[ptr][1] = 0;
                }
            }
            State::Zero => {
                if value == 0 {
                } else {
                    indicies[ptr][0] = i as i32;
                    indicies[ptr][1] = i as i32;
                }
            }
            State::One => {
                if value == 0 {
                    longest_value(&mut longest, &mut ptr, indicies)
                } else {
                    indicies[ptr][1] = i as i32;
                }
            }
        };

        if value == 0 {
            state = State::Zero;
            seen_zero = true;
        } else {
            state = State::One;
        }
    }

    longest_value(&mut longest, &mut ptr, indicies);

    if seen_zero {
        return longest;
    } else {
        return longest - 1;
    }
}

pub fn test_longest_subarray() {
    assert!(longest_subarray(vec![1, 1, 1]) == 2);
    assert!(longest_subarray(vec![1, 1, 0, 1]) == 3);
    assert!(longest_subarray(vec![0, 1, 1, 1, 0, 1, 1, 0, 1]) == 5);
    assert!(longest_subarray(vec![0]) == 0);
    assert!(longest_subarray(vec![1]) == 1);
    assert!(longest_subarray(vec![0, 0]) == 0);
    assert!(longest_subarray(vec![1, 1]) == 1);
    assert!(longest_subarray(vec![1, 0, 1]) == 2);
    assert!(longest_subarray(vec![1, 1, 0]) == 2);
    assert!(longest_subarray(vec![0, 0, 0, 0, 1, 1, 1, 0, 0, 0,]) == 3);
    assert!(longest_subarray(vec![1, 1, 0]) == 2);
    assert!(longest_subarray(vec![1, 0, 0, 1, 0]) == 1);
}
