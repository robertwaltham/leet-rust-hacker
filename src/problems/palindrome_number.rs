/*
Given an integer x, return true if x is a
palindrome
, and false otherwise.



Example 1:

Input: x = 121
Output: true
Explanation: 121 reads as 121 from left to right and from right to left.

Example 2:

Input: x = -121
Output: false
Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.

Example 3:

Input: x = 10
Output: false
Explanation: Reads 01 from right to left. Therefore it is not a palindrome.



Constraints:

    -2^31 <= x <= 2^31 - 1


Follow up: Could you solve it without converting the integer to a string?


 */

pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    } else if x < 10 {
        return true;
    } else {
        let radix: i32 = 10;
        let mut n = x;
        let mut reversed = 0;

        let mut i: u32 = 0;
        let mut j: u32 = (x.ilog10() + 1);

        while j - i > 1 {
            let first = x / radix.pow(i) % 10;
            let second = x / radix.pow(j - 1) % 10;

            if first != second {
                return false;
            }

            i += 1;
            j -= 1;
        }
        return true;
    }
}

pub fn test_is_palindrome() {
    assert!(is_palindrome(123456) == false);
    assert!(is_palindrome(1234567) == false);
    assert!(is_palindrome(123454321));

    assert!(is_palindrome(1001));
    assert!(is_palindrome(0));
    assert!(is_palindrome(10) == false);
    assert!(is_palindrome(121));
    assert!(is_palindrome(-121) == false);
}
