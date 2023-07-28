/*
The variance of a string is defined as the largest difference between the number of occurrences of any 2 characters present in the string. Note the two characters may or may not be the same.

Given a string s consisting of lowercase English letters only, return the largest variance possible among all substrings of s.

A substring is a contiguous sequence of characters within a string.



Example 1:

Input: s = "aababbb"
Output: 3
Explanation:
All possible variances along with their respective substrings are listed below:
- Variance 0 for substrings "a", "aa", "ab", "abab", "aababb", "ba", "b", "bb", and "bbb".
- Variance 1 for substrings "aab", "aba", "abb", "aabab", "ababb", "aababbb", and "bab".
- Variance 2 for substrings "aaba", "ababbb", "abbb", and "babb".
- Variance 3 for substring "babbb".
Since the largest possible variance is 3, we return it.

Example 2:

Input: s = "abcde"
Output: 0
Explanation:
No letter occurs more than once in s, so the variance of every substring is 0.



Constraints:

    1 <= s.length <= 10^4
    s consists of lowercase English letters.

 */

pub fn largest_variance(s: String) -> i32 {
    use std::collections::HashMap;

    let mut largest = 0;
    let mut running_count: HashMap<char, i32> = HashMap::new();
    let mut max_diff: HashMap<(char, char), i32> = HashMap::new();
    let mut max_diff_seen: HashMap<(char, char), i32> = HashMap::new();

    s.chars().enumerate().fold(0, |mut max, (i, char)| {
        *running_count.entry(char).or_insert(0) += 1;

        for other_char in 'a'..='z' {
            *max_diff_seen.entry((char, other_char)).or_insert(-10000) =
                *max_diff.entry((char, other_char)).or_insert(0);

            let count_char = running_count.get(&char).unwrap_or(&0);
            let count_other_char = running_count.get(&other_char).unwrap_or(&0);

            max_diff
                .entry((char, other_char))
                .and_modify(|e| *e = std::cmp::max(*e, *count_char - *count_other_char))
                .or_insert(std::cmp::max(0, count_char - count_other_char));

            let char_best = count_char - count_other_char
                + max_diff_seen.get(&(other_char, char)).unwrap_or(&-10000);
            let other_char_best = count_other_char - count_char
                + max_diff_seen.get(&(char, other_char)).unwrap_or(&-10000);

            max = std::cmp::max(max, char_best);
            max = std::cmp::max(max, other_char_best);
        }
        max
    })
}

pub fn largest_variance_bad(s: String) -> i32 {
    use std::collections::HashMap;

    if s.len() < 3 {
        return 0;
    }

    let occurrence = s.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    let max_occurrence = occurrence.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap();

    if *max_occurrence.1 < 2 {
        return 0;
    }

    let bounds = s.chars().fold((i32::MAX, -1, 0), |mut acc, c| {
        if c == *max_occurrence.0 {
            acc.0 = std::cmp::min(acc.0, acc.2);
            acc.1 = std::cmp::max(acc.0, acc.2);
        }
        acc.2 += 1;
        acc
    });

    print!("{max_occurrence:?} {bounds:?} | ");

    fn calc_variance(s: &str) -> i32 {
        let occurrences = s.chars().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });
        let minmax = occurrences.iter().fold((1, i32::MAX), |mut acc, c| {
            acc.0 = std::cmp::max(acc.0, *c.1);
            acc.1 = std::cmp::min(acc.1, *c.1);
            acc
        });
        print!("{s} {} | ", minmax.0 - minmax.1);
        minmax.0 - minmax.1
    }

    if bounds.0 == 0 && bounds.1 == s.len() as i32 - 1 {
        return calc_variance(&s[bounds.0 as usize..bounds.1 as usize + 1]);
    } else if bounds.0 == 0 {
        return std::cmp::max(
            calc_variance(&s[bounds.0 as usize..bounds.1 as usize + 1]),
            calc_variance(&s[bounds.0 as usize..bounds.1 as usize + 2]),
        );
    } else if bounds.1 == s.len() as i32 - 1 {
        return std::cmp::max(
            calc_variance(&s[bounds.0 as usize - 1..bounds.1 as usize + 1]),
            calc_variance(&s[bounds.0 as usize..bounds.1 as usize + 1]),
        );
    } else {
        let a = calc_variance(&s[bounds.0 as usize - 1..bounds.1 as usize + 1]);
        let b = calc_variance(&s[bounds.0 as usize..bounds.1 as usize + 1]);
        let c = calc_variance(&s[bounds.0 as usize..bounds.1 as usize + 2]);
        return std::cmp::max(a, std::cmp::max(b, c));
    }
}

pub fn largest_variance_brute_force(s: String) -> i32 {
    use std::collections::HashMap;

    fn calc_variance(s: &str) -> i32 {
        if s.len() == 1 {
            println!();
            return 0;
        }

        let occurrences = s.chars().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });
        let minmax = occurrences.iter().fold((1, i32::MAX), |mut acc, c| {
            acc.0 = std::cmp::max(acc.0, *c.1);
            acc.1 = std::cmp::min(acc.1, *c.1);
            acc
        });
        minmax.0 - minmax.1
    }

    let mut variance = 0;
    for i in 0..s.len() {
        for j in i + 1..s.len() {
            let slice = &s[i..j + 1];
            variance = std::cmp::max(calc_variance(slice), variance);
        }
    }
    variance
}

pub fn test_largest_variance() {
    fn test_case(s: &String, expected: i32) {
        let calculated = largest_variance(s.to_string());
        println!(
            "{} expected: {} calculated: {} match: {}",
            s,
            expected,
            calculated,
            expected == calculated
        );
    }

    test_case(&"abcde".to_string(), 0);
    test_case(&"aababbb".to_string(), 3);
    test_case(&"aaaaaabbbbbcccdde".to_string(), 5);
    test_case(&"aaaa".to_string(), 0);
    test_case(&"srndawsjtkfjvkgrfqkovajfbvlhqpoxzmtffmlrwwevtixyksauepdilfyuabdundhlkrbmmeppxslyhnumekdqcsqpmlcjsyctqebxsbvpapbmlqhrddpdthaboqokljnlbtyaqpumlzncdjqazugsinxwhcmxvtuiclmjqcsbabuxadnivdvvrvxygxlrrlummxlnasjrkqbhtuutiakfkwmfbtoxqbzhhvdlkylxrtcfqgwhcxotklbvfpjmeshlxfzookpharvrgqmwodlhrwcoxgbkpkvxbdffczbqnjfvxyvijoiguvfjmadjphaworbwgmwiitphnaavpuywxepfdbygkbjiupvvpkdjfipjvrdtufofdyvzsecreyylsmxemucryrstlittgqpxaeurnxukramvoxfdqqtnwrmnxdxgcxwfsewgqbfoqjc".to_string(), 18);
    test_case(&"baaaab".to_string(), 3);
    test_case(&"icexiahccknibwuwgi".to_string(), 3);
    test_case(&"aabaabaa".to_string(), 4);
}
