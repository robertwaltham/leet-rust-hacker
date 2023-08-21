/*

An array arr is a mountain if the following properties hold:

    arr.length >= 3
    There exists some i with 0 < i < arr.length - 1 such that:
        arr[0] < arr[1] < ... < arr[i - 1] < arr[i]
        arr[i] > arr[i + 1] > ... > arr[arr.length - 1]

Given a mountain array arr, return the index i such that arr[0] < arr[1] < ... < arr[i - 1] < arr[i] > arr[i + 1] > ... > arr[arr.length - 1].

You must solve it in O(log(arr.length)) time complexity.

Example 1:

Input: arr = [0,1,0]
Output: 1

Example 2:

Input: arr = [0,2,1,0]
Output: 1

Example 3:

Input: arr = [0,10,5,2]
Output: 1

Constraints:

    3 <= arr.length <= 10^5
    0 <= arr[i] <= 10^6
    arr is guaranteed to be a mountain array.
*/

struct Solution;

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut low = 0_usize;
        let mut high = arr.len() - 1;
        let mut mid = 0;
        while high - low > 1 {
            mid = (low + high) / 2;
            println!("{} {} {}", low, mid, high);
            let left_foot = arr[mid];
            let right_foot = arr[mid - 1];
            if left_foot > right_foot {
                low = mid;
            } else {
                high = mid;
            }
        }

        if arr[high] > arr[low] {
            high as i32
        } else {
            low as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::mountain_array::Solution;
    fn test_case(arr: Vec<i32>, expected: i32) {
        println!("array: {:?} expected: {}", arr, expected);
        assert_eq!(expected, Solution::peak_index_in_mountain_array(arr));
    }

    #[test]
    pub fn test_peak_index_in_mountain_array() {
        test_case([1, 2, 5, 9].to_vec(), 3);
        test_case([4, 3, 2, 1].to_vec(), 0);
        test_case(
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6, 5, 4, 3, 2, 1].to_vec(),
            8,
        );
    }
}
