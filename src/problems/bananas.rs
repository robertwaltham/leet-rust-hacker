/*
Koko loves to eat bananas. There are n piles of bananas, the ith pile has piles[i] bananas. The guards have gone and will come back in h hours.

Koko can decide her bananas-per-hour eating speed of k. Each hour, she chooses some pile of bananas and eats k bananas from that pile. If the pile has less than k bananas, she eats all of them instead and will not eat any more bananas during this hour.

Koko likes to eat slowly but still wants to finish eating all the bananas before the guards return.

Return the minimum integer k such that she can eat all the bananas within h hours.



Example 1:

Input: piles = [3,6,7,11], h = 8
Output: 4

Example 2:

Input: piles = [30,11,23,4,20], h = 5
Output: 30

Example 3:

Input: piles = [30,11,23,4,20], h = 6
Output: 23

*/

/*
Notes:

- This looks like a 2 pointer problem
- WRONG this was a binary search problem
- trying to find the smallest entry such that k entries can be eaten in 1 hour, where k is h - piles.length
- h < piles.length is undefined


*/

pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    assert!(piles.len() <= h as usize);

    macro_rules! can_finish {
        ($k:expr) => {{
            let mut hours = h;
            for pile in &piles {
                hours -= (pile + $k - 1) / $k;
                if hours < 0 {
                    break;
                }
            }
            hours >= 0
        }};
    }

    let mut high = *piles.iter().max().unwrap();
    let mut low = 1;

    while low < high {
        let k = (low + high) / 2;
        if can_finish!(k) {
            high = k;
        } else {
            low += 1;
        }
    }

    low
}

pub fn test_min_eating_speed() {
    fn test_case(piles: Vec<i32>, h: i32, expected: i32) {
        let result = min_eating_speed(piles.to_vec(), h);
        if result == expected {
            println!(
                "Passed - piles:{:?} h:{:?} expected: {:?}",
                piles, h, expected
            );
        } else {
            println!(
                "Failed - piles:{:?} h:{:?} expected: {:?} - got {:?}",
                piles, h, expected, result
            );
        }
    }

    test_case([3, 6, 7, 11].to_vec(), 8, 4);
    test_case([30, 11, 23, 4, 20].to_vec(), 5, 30);
    test_case([30, 11, 23, 4, 20].to_vec(), 6, 23);
    test_case([30, 11, 23, 4, 20].to_vec(), 7, 20);
}
