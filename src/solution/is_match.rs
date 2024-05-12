impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        // 准备
        let star = '*' as u8;
        let any_byte = '.' as u8;
        let sb = s.as_bytes();
        let pb = p.as_bytes();
        let s_len = sb.len();
        let p_len = pb.len();

        // 初始化状态数组
        let mut macth_status: Vec<Vec<bool>> = Vec::new();
        macth_status.resize(s_len + 1, Vec::new());
        for i in &mut macth_status {
            i.resize(p_len + 1, false);
        }
        macth_status[0][0] = true;

        // 判断第 `i - 1` 个字符和第 `j - 1` 个字符是否匹配
        let is_match = |i, j| {
            if i == 0 && j != 0 {
                return false;
            }
            return pb[j - 1] == any_byte || sb[i - 1] == pb[j - 1];
        };

        for i in 0..=s_len {
            for j in 1..=p_len {
                if pb[j - 1] == star {
                    macth_status[i][j] = macth_status[i][j - 2]; // 题目保证了 * 的前面有字符, 所以这里 j >= 2
                    if is_match(i, j - 1) {
                        macth_status[i][j] |= macth_status[i - 1][j]; // is_match 保证了这里 i >= 1
                    }
                } else if is_match(i, j) {
                    macth_status[i][j] = macth_status[i - 1][j - 1];
                }
            }
        }

        return macth_status[s_len][p_len];
    }
}

struct Solution;

#[cfg(test)]
mod test {
    use crate::solution::is_match;

    #[test]
    fn it_work() {
        fn t(s: &str, p: &str, pre: bool) {
            let ret = is_match::Solution::is_match(s.to_string(), p.to_string());
            assert_eq!(ret, pre)
        }

        t("aa", "a", false);
        t("aa", "a*", true);
        t("ab", ".*", true);
    }
}
