/*
Implement the myAtoi(string s) function, which converts a string to a 32-bit signed integer (similar to C/C++'s atoi function).

The algorithm for myAtoi(string s) is as follows:

    Read in and ignore any leading whitespace.
    Check if the next character (if not already at the end of the string) is '-' or '+'. Read this character in if it is either. This determines if the final result is negative or positive respectively. Assume the result is positive if neither is present.
    Read in next the characters until the next non-digit character or the end of the input is reached. The rest of the string is ignored.
    Convert these digits into an integer (i.e. "123" -> 123, "0032" -> 32). If no digits were read, then the integer is 0. Change the sign as necessary (from step 2).
    If the integer is out of the 32-bit signed integer range [-2^31, 2^31 - 1], then clamp the integer so that it remains in the range. Specifically, integers less than -231 should be clamped to -231, and integers greater than 231 - 1 should be clamped to 231 - 1.
    Return the integer as the final result.

Note:

    Only the space character ' ' is considered a whitespace character.
    Do not ignore any characters other than the leading whitespace or the rest of the string after the digits.

 */

pub fn my_atoi(s: String) -> i32 {
    #[derive(Debug)]
    struct R {
        i: i32,
        start: Option<i32>,
        end: Option<i32>,
        negative: Option<bool>,
        state: State,
    }

    #[derive(Debug)]
    enum State {
        Whitespace,
        Sign,
        LeadingZeros,
        Digits,
        Done,
    }

    impl R {
        fn init() -> R {
            R {
                i: -1,
                start: None,
                end: None,
                negative: None,
                state: State::Whitespace,
            }
        }
    }

    let slice = s.as_str();
    let result = slice
        .as_bytes()
        .iter()
        .fold(R::init(), |mut acc, character| {
            acc.i += 1;
            return match acc.state {
                State::Done => acc,
                State::Whitespace => {
                    match character {
                        45 => {
                            acc.negative = Some(true);
                            acc.state = State::Sign
                        } // -
                        43 => {
                            acc.negative = Some(false);
                            acc.state = State::Sign
                        } // +
                        32 => {}                               // Space, continue
                        48 => acc.state = State::LeadingZeros, // Leading Zero
                        49..=57 => {
                            acc.state = State::Digits;
                            acc.start = Some(acc.i);
                            acc.end = Some(acc.i);
                        } // Other Digits
                        _ => acc.state = State::Done, // non-space, non-sign character, we're done
                    }
                    acc
                }
                State::Sign => {
                    match character {
                        48 => acc.state = State::LeadingZeros, // Leading Zero
                        49..=57 => {
                            acc.state = State::Digits;
                            acc.start = Some(acc.i);
                            acc.end = Some(acc.i);
                        } // Other Digits
                        _ => acc.state = State::Done,
                    }
                    acc
                }
                State::LeadingZeros => {
                    match character {
                        48 => {} // Leading Zero
                        49..=57 => {
                            acc.state = State::Digits;
                            acc.start = Some(acc.i);
                            acc.end = Some(acc.i);
                        } // Other Digits
                        _ => acc.state = State::Done,
                    }
                    acc
                }
                State::Digits => {
                    match character {
                        48..=57 => {
                            acc.end = Some(acc.i);
                        } // Other Digits
                        _ => acc.state = State::Done,
                    }
                    acc
                }
            };
        });
    if result.start == None || result.end == None {
        0
    } else {
        let number = match slice[result.start.unwrap() as usize..=result.end.unwrap() as usize]
            .as_bytes()
            .iter()
            .rev()
            .fold((Some(0_i32), 0_u32), |mut acc, byte| {
                let value = (byte - 48) as i32;
                if acc.0 == None {
                    acc.1 += 1;
                    return acc;
                }
                acc.0 = match 10_i32.checked_pow(acc.1) {
                    Some(i) => match i.checked_mul(value) {
                        Some(j) => acc.0.unwrap().checked_add(j),
                        None => None,
                    },
                    None => None,
                };
                acc.1 += 1;
                acc
            })
            .0
        {
            Some(n) => {
                if result.negative == Some(true) {
                    -n
                } else {
                    n
                }
            }
            None => {
                if result.negative == Some(true) {
                    i32::MIN
                } else {
                    i32::MAX
                }
            }
        };
        number
    }
}

pub fn my_atoi_attempt_1(s: String) -> i32 {
    // println!("Converting: {}", s);

    let result = s.as_str().as_bytes().iter().rev().fold(
        (0_u32, 0_i64, 1, false, false),
        |mut acc, item| {
            match item {
                48..=57 => {
                    // print!("digit: {} ", item - 48);
                    if acc.3 == true {
                        acc.3 = false;
                        acc.1 = 0;
                        acc.0 = 0;
                    }
                    acc.2 = 1;
                    if acc.0 < 19 {
                        match (*item as i64 - 48).checked_mul(10_i64.pow(acc.0)) {
                            Some(j) => {
                                acc.1 += j;
                                acc.0 += 1;
                            }
                            None => {}
                        }
                    } else {
                        acc.1 = 2147483648;
                    }

                    // print!("acc: {:?} ", acc);
                }
                32 => {
                    /* print!("space "); */
                    if acc.0 > 0 {
                        acc.3 = true
                    }
                }
                45 => {
                    print!("- ");
                    if acc.0 > 0 {
                        acc.3 = true
                    };
                    if !acc.4 {
                        acc.2 = -1;
                        acc.4 = true
                    } else {
                        acc.1 = 0
                    }
                }
                43 => {
                    print!("+ ");
                    if acc.0 > 0 {
                        acc.3 = true
                    };
                    if !acc.4 {
                        acc.2 = 1;
                        acc.4 = true
                    } else {
                        acc.1 = 0
                    }
                }
                _ => acc = (0, 0, 1, false, acc.4),
            }
            acc
        },
    );
    println!("result {:?}", result);
    match i32::try_from(result.1) {
        Ok(casted_result) => casted_result * result.2,
        Err(err) => {
            if result.2 == 1 {
                i32::MAX
            } else {
                i32::MIN
            }
        }
    }
}

pub fn test_my_atoi() {
    assert!(my_atoi("12345".to_string()) == 12345);
    assert!(my_atoi("-12345".to_string()) == -12345);
    assert!(my_atoi("+12345".to_string()) == 12345);
    assert!(my_atoi("-0".to_string()) == -0);
    assert!(my_atoi("0".to_string()) == 0);
    assert!(my_atoi("-12345somegarbage".to_string()) == -12345);
    assert!(my_atoi("      12345".to_string()) == 12345);
    assert!(my_atoi("00012345".to_string()) == 12345);
    assert!(my_atoi("2147483648".to_string()) == i32::MAX);
    assert!(my_atoi("-2147483649".to_string()) == i32::MIN);
    assert!(my_atoi("somegarbage".to_string()) == 0);
    assert!(my_atoi("somegarbage1234".to_string()) == 0);
    assert!(my_atoi("1234    ".to_string()) == 1234);
    assert!(my_atoi("1234-    ".to_string()) == 1234);
    assert!(my_atoi("123   45".to_string()) == 123);
    assert!(my_atoi("+-12".to_string()) == 0);
    assert!(my_atoi("00000-42a1234".to_string()) == 0);
    assert!(my_atoi("20000000000000000000".to_string()) == i32::MAX);
    assert!(my_atoi("-000000000000000000000000000000000000000000000000001".to_string()) == -1);
}
