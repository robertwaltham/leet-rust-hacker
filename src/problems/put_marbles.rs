/*

You have k bags. You are given a 0-indexed integer array weights where weights[i] is the weight of the ith marble. You are also given the integer k.

Divide the marbles into the k bags according to the following rules:

    No bag is empty.
    If the ith marble and jth marble are in a bag, then all marbles with an index between the ith and jth indices should also be in that same bag.
    If a bag consists of all the marbles with an index from i to j inclusively, then the cost of the bag is weights[i] + weights[j].

The score after distributing the marbles is the sum of the costs of all the k bags.

Return the difference between the maximum and minimum scores among marble distributions.



Example 1:

Input: weights = [1,3,5,1], k = 2
Output: 4
Explanation:
The distribution [1],[3,5,1] results in the minimal score of (1+1) + (3+1) = 6.
The distribution [1,3],[5,1], results in the maximal score of (1+3) + (5+1) = 10.
Thus, we return their difference 10 - 6 = 4.

Example 2:

Input: weights = [1, 3], k = 2
Output: 0
Explanation: The only distribution possible is [1],[3].
Since both the maximal and minimal score are the same, we return 0.



Constraints:

    1 <= k <= weights.length <= 10^5
    1 <= weights[i] <= 10^9

 */

pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
    use std::cmp::{Ordering, Reverse};
    use std::collections::BinaryHeap;

    if k >= weights.len() as i32 {
        return 0;
    }

    let mut bars_max: BinaryHeap<i32> = BinaryHeap::new();
    let mut bars_min: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

    for i in 0..weights.len() - 1 {
        bars_max.push(weights[i] + weights[i + 1]);
        bars_min.push(Reverse(weights[i] + weights[i + 1]));
    }

    let mut min = (weights[0] + weights[weights.len() - 1]) as i64;
    let mut max = min;

    for i in 0..k - 1 {
        min += bars_min.pop().unwrap().0 as i64;
        max += bars_max.pop().unwrap() as i64;
    }

    return max - min;
}

pub fn put_marbles_attepmt_1(weights: Vec<i32>, k: i32) -> i64 {
    use std::cmp::{Ordering, Reverse};
    use std::collections::BinaryHeap;
    use std::fmt;

    #[derive(Eq)]
    struct Weight {
        index: usize,
        weight: i32,
    }
    impl Ord for Weight {
        fn cmp(&self, other: &Self) -> Ordering {
            self.weight.cmp(&other.weight)
        }
    }
    impl PartialOrd for Weight {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    impl PartialEq for Weight {
        fn eq(&self, other: &Self) -> bool {
            self.weight == other.weight
        }
    }
    impl fmt::Debug for Weight {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.weight)
        }
    }

    if k >= weights.len() as i32 {
        return 0;
    }

    let mut max_heap: BinaryHeap<Weight> = BinaryHeap::new();
    let mut min_heap: BinaryHeap<Reverse<Weight>> = BinaryHeap::new();

    for (i, weight) in weights.iter().enumerate() {
        max_heap.push(Weight {
            index: i,
            weight: *weight,
        });
        min_heap.push(Reverse(Weight {
            index: i,
            weight: *weight,
        }));
    }

    let mut min_list: Vec<Weight> = Vec::new();
    let mut max_list: Vec<Weight> = Vec::new();

    // min_list.push(Weight {index: 0, weight: weights[0]});
    // max_list.push(Weight {index: 0, weight: weights[0]});

    for i in 0..k {
        min_list.push(min_heap.pop().unwrap().0);
        max_list.push(max_heap.pop().unwrap());
    }

    // min_list.push(Weight {index: weights.len(), weight: weights[weights.len()-1]});
    // max_list.push(Weight {index: weights.len(), weight: weights[weights.len()-1]});

    min_list.sort_by(|a, b| a.index.cmp(&b.index));
    max_list.sort_by(|a, b| a.index.cmp(&b.index));

    println!("min: {:?}", min_list);
    println!("max: {:?}", max_list);

    // assert!(min_list.len() == (k * 2) as usize);
    // assert!(min_list.len() % 2 == 0);
    // assert!(max_list.len() % 2 == 0);

    let mut min_sum = 0_i64;
    let mut max_sum = 0_i64;

    for i in 0..min_list.len() / 2 {
        let min_weight_0 = min_list[i].weight as i64;
        let min_weight_1 = min_list[i].weight as i64;
        min_sum += min_weight_0 + min_weight_1;

        let max_weight_0 = max_list[i].weight as i64;
        let max_weight_1 = max_list[i].weight as i64;
        max_sum += max_weight_0 + max_weight_1;
    }

    return max_sum - min_sum;
}

pub fn test_put_marbles() {
    {
        let weights = [0, 1];
        let k = 2;
        let expected_result = 0;
        let result = put_marbles(weights.to_vec(), k);
        println!(
            "weights: {:?} k:{} expected:{} got:{} pass:{}",
            weights,
            k,
            expected_result,
            result,
            expected_result == result
        );
    }

    {
        let weights = [1, 3, 5, 1];
        let k = 2;
        let expected_result = 4;
        let result = put_marbles(weights.to_vec(), k);
        println!(
            "weights: {:?} k:{} expected:{} got:{} pass:{}",
            weights,
            k,
            expected_result,
            result,
            expected_result == result
        );
    }

    // {
    //     let weights = [1,3,5,1,3,2,4,1,3,2,4,1];
    //     let k = 4;
    //     let expected_result = 12;
    //     let result = put_marbles(weights.to_vec(), k);
    //     println!("weights: {:?} k:{} expected:{} got:{} pass:{}", weights, k, expected_result, result, expected_result == result);
    // }

    {
        let weights = [1, 4, 2, 5, 2];
        let k = 3;
        let expected_result = 3;
        let result = put_marbles(weights.to_vec(), k);
        println!(
            "weights: {:?} k:{} expected:{} got:{} pass:{}",
            weights,
            k,
            expected_result,
            result,
            expected_result == result
        );

        /*
            [1, 4, 2], [5], [2] = 17   5, 4, 2
            [1], [4, 2], [5, 2] = 15
            [1, 4], [2, 5], [2] = 14
            [1], [4], [2, 5, 2] = 14

        */
    }
}
