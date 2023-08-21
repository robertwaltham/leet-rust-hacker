/*
Given an array of points where points[i] = [xi, yi] represents a point on the X-Y plane and an integer k, return the k closest points to the origin (0, 0).

The distance between two points on the X-Y plane is the Euclidean distance (i.e., √(x1 - x2)2 + (y1 - y2)2).

You may return the answer in any order. The answer is guaranteed to be unique (except for the order that it is in).
*/

struct Solution;

#[derive(Eq, PartialEq, Debug)]
struct Point(i32, i32);

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};
impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        let distance = self.0.pow(2) + self.1.pow(2);
        let other_distance = other.0.pow(2) + other.1.pow(2);
        return distance.cmp(&other_distance).reverse();
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Point {
    fn to_vec(self) -> Vec<i32> {
        return [self.0, self.1].to_vec();
    }
}

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut map: BinaryHeap<Point> =
            points.into_iter().fold(BinaryHeap::new(), |mut sum, x| {
                sum.push(Point(x[0], x[1]));
                sum
            });
        let mut result: Vec<Vec<i32>> = Vec::new();
        for i in 0..k {
            result.push(map.pop().unwrap().to_vec())
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::k_closest_points::Solution;
    fn test_case(arr: Vec<Vec<i32>>, k: i32, expected: Vec<Vec<i32>>) {
        println!("array: {:?} k:{} expected: {:?}", arr, k, expected);
        assert_eq!(expected, Solution::k_closest(arr, k));
    }

    #[test]
    pub fn test_k_closest() {
        test_case(
            [[1, 3].to_vec(), [-2, 2].to_vec()].to_vec(),
            1,
            [[-2, 2].to_vec()].to_vec(),
        );
        test_case(
            [[3, 3].to_vec(), [5, -1].to_vec(), [-2, 4].to_vec()].to_vec(),
            2,
            [[3, 3].to_vec(), [-2, 4].to_vec()].to_vec(),
        );
    }
}
