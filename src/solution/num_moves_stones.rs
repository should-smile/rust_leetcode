impl Solution {
    pub fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
        use std::cmp::{max, min};
        let max_v = max(a, max(b, c));
        let min_v = min(a, min(b, c));
        let mid_v = a + b + c - min_v - max_v;

        let ans = vec![
            if min_v + 1 == mid_v && mid_v + 1 == max_v {
                0
            } else if mid_v - min_v <= 2 || max_v - mid_v <= 2 {
                1
            } else {
                2
            },
            max_v - min_v - 2,
        ];

        return ans;
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use crate::solution::num_moves_stones::Solution;

    #[test]
    fn it_works() {
        fn t(a: i32, b: i32, c: i32, expectation: Vec<i32>) {
            let ans = Solution::num_moves_stones(a, b, c);
            assert_eq!(ans, expectation);
        }

        t(1, 2, 5, vec![1, 2]);
        t(4, 3, 2, vec![0, 0]);
    }
}
