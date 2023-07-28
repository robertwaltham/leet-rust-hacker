/*

You are given an integer array height of length n. There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).

Find two lines that together with the x-axis form a container, such that the container contains the most water.

Return the maximum amount of water a container can store.

Notice that you may not slant the container.

 */

pub fn max_area(height: Vec<i32>) -> i32 {
    if height.len() < 2 {
        return 0;
    }

    if height.len() == 2 {
        return i32::min(height[0], height[1]);
    }

    let mut i = 0_i32;
    let mut j = height.len() as i32 - 1;
    let mut max = 0;

    fn calc_area(i: i32, j: i32, height: &Vec<i32>) -> i32 {
        (j - i) * i32::min(height[i as usize], height[j as usize])
    }

    while i < j {
        let area = calc_area(i, j, &height);
        if area > max {
            max = area;
        }
        if height[i as usize] < height[j as usize] {
            i += 1;
        } else {
            j -= 1;
        }
    }
    max
}

pub fn max_area_brute_force(height: Vec<i32>) -> i32 {
    let mut max = 0_i32;

    for (i, a) in height.iter().enumerate() {
        for (j, b) in height.iter().enumerate() {
            let area = i32::abs(i as i32 - j as i32) * i32::min(*a, *b);
            if area > max {
                max = area
            }
        }
    }

    max
}

pub fn test_max_area() {
    fn test_case(height: &Vec<i32>) {
        let brute_force = max_area_brute_force(height.to_vec());
        let max_area = max_area(height.to_vec());
        println!(
            "{:?} brute_force:{} calculated:{} match:{}",
            height,
            brute_force,
            max_area,
            brute_force == max_area
        );
    }

    test_case(&[1, 8, 6, 2, 5, 4, 8, 3, 7].to_vec());
    test_case(&[2, 3, 9, 8, 5, 7, 9, 2, 3, 8, 5, 7].to_vec());
    test_case(&[1, 1, 9, 1, 9, 1, 1].to_vec());
    test_case(&[1, 9, 1, 9, 1, 1, 1].to_vec());
    test_case(&[1, 1, 2, 8, 1, 8, 2, 1, 1].to_vec());

    test_case(&[2].to_vec());
    test_case(&[2, 3].to_vec());
    test_case(&Vec::new());
}
