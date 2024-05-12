impl Solution {
    pub fn one_edit_away(first: String, second: String) -> bool {
        if first.len().abs_diff(second.len()) > 1 {
            return false;
        }

        let s1;
        let s2;
        if first.len() <= second.len() {
            s1 = first.as_bytes();
            s2 = second.as_bytes();
        } else {
            s1 = second.as_bytes();
            s2 = first.as_bytes();
        }

        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut flag = true;
        while i < s1.len() && j < s2.len() {
            if s1[i] == s2[j] {
                i += 1;
                j += 1;
                continue;
            }

            if !flag {
                return false;
            }

            flag = false;

            if s1.len() == s2.len() {
                i += 1;
                j += 1;
            } else {
                j += 1;
            }
        }

        return true;
    }
}

struct Solution;

#[cfg(test)]
mod t {
    use crate::solution::one_edit_away;

    #[test]
    fn it_work() {
        fn t(first: &str, second: &str, pre: bool) {
            let res = one_edit_away::Solution::one_edit_away(first.to_string(), second.to_string());
            assert_eq!(res, pre);
        }

        t("pale", "ple", true);
        t("pales", "pal", false);
    }
}
