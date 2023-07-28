/*

Given an integer array nums, rotate the array to the right by k steps, where k is non-negative.



Example 1:

Input: nums = [1,2,3,4,5,6,7], k = 3
Output: [5,6,7,1,2,3,4]
Explanation:
rotate 1 steps to the right: [7,1,2,3,4,5,6]
rotate 2 steps to the right: [6,7,1,2,3,4,5]
rotate 3 steps to the right: [5,6,7,1,2,3,4]

Example 2:

Input: nums = [-1,-100,3,99], k = 2
Output: [3,99,-1,-100]
Explanation:
rotate 1 steps to the right: [99,-1,-100,3]
rotate 2 steps to the right: [3,99,-1,-100]



Constraints:

    1 <= nums.length <= 10^5
    -2^31 <= nums[i] <= 2^31 - 1
    0 <= k <= 10^5



Follow up:

    Try to come up with as many solutions as you can. There are at least three different ways to solve this problem.
    Could you do it in-place with O(1) extra space?



 */

pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    if k == 0 {
        return;
    }

    let mut cache = nums[0];
    let mut ptr = 0_usize;
    let mut start = 0_usize;
    let mut start_value = nums[0];
    let len: usize;

    let has_cycle = nums.len() % k as usize == 0;

    for i in 0..nums.len() {
        let j = cache;
        ptr = (ptr + k as usize) % nums.len();

        cache = nums[ptr as usize];
        nums[ptr as usize] = j;

        if start_value == cache && start == ptr {
            ptr = (1 + ptr) % nums.len();
            cache = nums[ptr];
            start = ptr;
            start_value = cache;
        }
    }
}

pub fn test_rotate() {
    let mut test_1: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
    print!("{:?} => ", test_1);
    rotate(&mut test_1, 3);
    println!("{:?}", test_1);
    assert!(test_1 == vec![5, 6, 7, 1, 2, 3, 4]);

    let mut test_2: Vec<i32> = vec![1, 2, 3, 4];
    print!("{:?} => ", test_2);
    rotate(&mut test_2, 4);
    println!("{:?}", test_2);
    assert!(test_2 == vec![1, 2, 3, 4]);

    let mut test_3: Vec<i32> = vec![1, 2, 3, 4];
    print!("{:?} => ", test_3);
    rotate(&mut test_3, 3);
    println!("{:?}", test_3);
    assert!(test_3 == vec![2, 3, 4, 1]);

    let mut test_4: Vec<i32> = vec![1, 2, 3, 4, 5];
    print!("{:?} => ", test_4);
    rotate(&mut test_4, 2);
    println!("{:?}", test_4);
    assert!(test_4 == vec![4, 5, 1, 2, 3]);

    let mut test_5: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    print!("{:?} => ", test_5);
    rotate(&mut test_5, 2);
    println!("{:?}", test_5);
    assert!(test_5 == vec![5, 6, 1, 2, 3, 4]);

    let mut test_6: Vec<i32> = vec![1, 2, 3, 4];
    print!("{:?} => ", test_6);
    rotate(&mut test_6, 2);
    println!("{:?}", test_6);
    assert!(test_6 == vec![3, 4, 1, 2]);

    let mut test_7: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    print!("{:?} => ", test_7);
    rotate(&mut test_7, 3);
    println!("{:?}", test_7);
    assert!(test_7 == vec![4, 5, 6, 1, 2, 3]);

    let mut test_8: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    print!("{:?} => ", test_8);
    rotate(&mut test_8, 4);
    println!("{:?}", test_8);
    assert!(test_8 == vec![3, 4, 5, 6, 1, 2]);
}
