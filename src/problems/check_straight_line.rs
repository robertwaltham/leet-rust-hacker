/*
You are given an array coordinates, coordinates[i] = [x, y], where [x, y] represents the coordinate of a point. Check if these points make a straight line in the XY plane.

Example 1:

Input: coordinates = [[1,2],[2,3],[3,4],[4,5],[5,6],[6,7]]
Output: true

Example 2:

Input: coordinates = [[1,1],[2,2],[3,4],[4,5],[5,6],[7,7]]
Output: false

Constraints:

    2 <= coordinates.length <= 1000
    coordinates[i].length == 2
    -10^4 <= coordinates[i][0], coordinates[i][1] <= 10^4
    coordinates contains no duplicate point.


 */

pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
    println!("{:?}", coordinates);
    let first = coordinates.first().unwrap();
    let mut slope: f32 = 0.;
    for (i, coord) in coordinates.iter().enumerate() {
        if i > 0 {
            let newslope: f32 = (coord[1] - first[1]) as f32 / (coord[0] - first[0]) as f32;
            if i > 1 {
                println!("slope: {} newslope: {}", slope, newslope);
                if newslope != slope {
                    if newslope.is_finite() || slope.is_finite() {
                        return false;
                    }
                }
            }
            slope = newslope;
        }
    }
    true
}

pub fn test_check_straight_line() {
    assert!(
        check_straight_line(vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![6, 7]
        ]) == true
    );
    assert!(
        check_straight_line(vec![
            vec![1, 1],
            vec![2, 2],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![7, 7]
        ]) == false
    );
    assert!(
        check_straight_line(vec![
            vec![4, 8],
            vec![-2, 8],
            vec![1, 8],
            vec![8, 8],
            vec![-5, 8],
            vec![0, 8],
            vec![7, 8],
            vec![5, 8]
        ]) == true
    );
    assert!(check_straight_line(vec![vec![0, 0], vec![0, 1], vec![0, -1]]) == true);
}
