impl Solution {
    pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut stack: Vec<(i32, usize)> = Vec::new();

        let mut p = head;
        let mut idx = 0;
        while let Some(v) = p {
            ans.push(0);
            while let Some(last) = stack.last() {
                if last.0 >= v.val {
                    break;
                }

                ans[last.1] = v.val;
                stack.pop();
            }

            stack.push((v.val, idx));
            idx += 1;
            p = v.next;
        }

        return ans;
    }
}

use super::commen_def::ListNode;
pub struct Solution;

#[cfg(test)]
mod t {
    use crate::solution::{commen_def::ListNode, next_larger_nodes::Solution};

    #[test]
    fn it_works() {
        fn t(data: Vec<i32>, pre:Vec<i32>) {
            let head = ListNode::from_vec(data);
            let ans = Solution::next_larger_nodes(head);
            assert_eq!(ans, pre);
        }

        t(vec![2, 1, 5], vec![5, 5, 0]);
        t(vec![2, 7, 4, 3, 5], vec![7, 0, 5, 5, 0]);
        t(vec![], vec![]);
        t(vec![1,7,5,1,9,2,5,1], vec![7,9,9,9,0,5,0,0]);
    }
}