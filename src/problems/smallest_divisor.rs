/*

Given an array of integers nums and an integer threshold, we will choose a positive integer divisor, divide all the array by it, and sum the division's result. Find the smallest divisor such that the result mentioned above is less than or equal to threshold.

Each result of the division is rounded to the nearest integer greater than or equal to that element. (For example: 7/3 = 3 and 10/2 = 5).

The test cases are generated so that there will be an answer.


Example 1:

Input: nums = [1,2,5,9], threshold = 6
Output: 5
Explanation: We can get a sum to 17 (1+2+5+9) if the divisor is 1.
If the divisor is 4 we can get a sum of 7 (1+1+2+3) and if the divisor is 5 the sum will be 5 (1+1+1+2).

Example 2:

Input: nums = [44,22,33,11,1], threshold = 5
Output: 44


Constraints:

    1 <= nums.length <= 5 * 10^4
    1 <= nums[i] <= 10^6
    nums.length <= threshold <= 10^6

*/

pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
    let mut high = *nums.iter().max().unwrap();
    let mut low = 1;

    while low < high {
        let mid = (high + low) / 2;
        let sum = nums
            .iter()
            .fold(0, |sum, i| sum + (i / mid) + (i % mid).signum());

        if sum > threshold {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    low
}

#[cfg(test)]
mod tests {

    fn test_case(nums: Vec<i32>, threshold: i32, expected: i32) {
        println!(
            "nums: {:?} threshold: {} expected: {}",
            nums, threshold, expected
        );
        assert_eq!(
            crate::problems::smallest_divisor::smallest_divisor(nums, threshold),
            expected
        );
    }

    #[test]
    pub fn test_smallest_divisor() {
        test_case([1, 2, 5, 9].to_vec(), 6, 5);
    }
}
