impl Solution {
    pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = Vec::new();
        ans.resize(queries.len(), 0);

        for point in points {
            for i in 0..queries.len() {
                let x = (point[0] - queries[i][0]) * (point[0] - queries[i][0]);
                let y = (point[1] - queries[i][1]) * (point[1] - queries[i][1]);
                if x + y <= queries[i][2] * queries[i][2] {
                    ans[i] += 1;
                }
            }
        }

        return ans;
    }
}

struct Solution;

#[cfg(test)]
mod test {
    use crate::{solution::count_points, vecnd};
    #[test]
    fn it_work() {
        fn t(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>, pre: Vec<i32>) {
            let ret = count_points::Solution::count_points(points, queries);
            assert_eq!(ret, pre)
        }

        t(
            vecnd![[1, 3], [3, 3], [5, 3], [2, 2]],
            vecnd![[2, 3, 1], [4, 3, 1], [1, 1, 2]],
            vec![3, 2, 2],
        );

        t(
            vecnd![[1, 1], [2, 2], [3, 3], [4, 4], [5, 5]],
            vecnd![[1, 2, 2], [2, 2, 2], [4, 3, 2], [4, 3, 3]],
            vec![2, 3, 2, 4],
        );
    }
}
