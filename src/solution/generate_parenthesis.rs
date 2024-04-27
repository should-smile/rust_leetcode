struct Solution;
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn g(n: i32, ln: i32, rn: i32, s: &mut Vec<char>, ss: &mut Vec<String>) {
            if ln + rn >= n + n {
                ss.push(s.iter().collect());
                return;
            }

            if ln < n {
                s.push('(');
                g(n, ln + 1, rn, s, ss);
                s.pop();
            }

            if rn < ln && rn < ln {
                s.push(')');
                g(n, ln, rn + 1, s, ss);
                s.pop();
            }
        }
        let mut res = Vec::new();
        let mut s = Vec::new();
        g(n, 0, 0, &mut s, &mut res);
        return res;
    }
}