impl Solution {
    pub fn reverse_bits(num: i32) -> i32 {
        let mut ans: i32 = 0;
        let mut cur_len: i32 = 0;
        let mut cur_insert_len: i32 = 0;

        let mut cur_num = num;
        for i in 0..32 {
            if (cur_num & 1) != 0 {
                cur_insert_len += 1;
                cur_len += 1;
            } else {
                cur_insert_len = cur_len + 1;
                cur_len = 0;
            }
            ans = std::cmp::max(ans, cur_insert_len);

            cur_num >>= 1;
        }
        return ans;
    }
}

struct Solution;

#[cfg(test)]
mod test {
    use crate::solution::reverse_bits;

    #[test]
    fn it_work() {
        fn t(num: i32, pre: i32) {
            let res = reverse_bits::Solution::reverse_bits(num);
            assert_eq!(res, pre);
        }

        // t(0, 0);
        t(1775, 8);
        t(7, 4);
    }
}
