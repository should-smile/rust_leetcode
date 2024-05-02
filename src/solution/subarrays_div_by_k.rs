impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;

        let mut record = HashMap::new();
        record.insert(0, 1);

        let mut sum = 0;
        let mut ans = 0;
        for num in nums {
            sum += num;
            let modulus = (sum % k + k) % k;

            let count = record.entry(modulus).or_insert(0);
            ans += *count;
            *count += 1;

        }
        return ans;
    }
}

struct Solution;

#[cfg(test)]
mod test {
    use crate::solution::subarrays_div_by_k::Solution;

    #[test]
    fn it_work() {
        fn t(nums: Vec<i32>, k: i32, pre: i32) {
            let ans = Solution::subarrays_div_by_k(nums, k);
            assert_eq!(ans, pre);
        }

        t(vec![4, 5, 0, -2, -3, 1], 5, 7);
        t(vec![5], 9, 0);
        t(vec![], 0, 0);
    }
}
