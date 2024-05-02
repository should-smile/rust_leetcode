use super::commen_def::TreeNode;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root == None {
            return 0;
        }

        let max_size = 1e4 as usize;
        let mut record = HashMap::with_capacity(max_size);

        let mut stack = Vec::with_capacity(max_size);
        stack.push((1, root.unwrap()));
        while let Some(cur) = stack.pop() {
            let n = &cur.1.borrow();
            *record.entry(cur.0).or_insert(0) += n.val;

            if n.left != None {
                stack.push((cur.0 + 1, n.left.as_ref().unwrap().clone()));
            }
            if n.right != None {
                stack.push((cur.0 + 1, n.right.as_ref().unwrap().clone()));
            }
        }

        let ans = record.iter().enumerate().max_by(|i1, i2| {
            if i1.1 .1 == i2.1 .1 {
                i1.1 .0.cmp(i2.1 .0).reverse()
            } else {
                i1.1 .1.cmp(i2.1 .1)
            }
        });
        return *ans.unwrap().1 .0;
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use crate::solution::{commen_def::TreeNode, max_level_sum::Solution};

    #[test]
    fn it_works() {
        fn t(data: &str, exp: i32) {
            let root = TreeNode::from_str(data);
            let ans = Solution::max_level_sum(root);
            assert_eq!(ans, exp);
        }

        t("[1,7,0,7,-8,null,null]", 2);
        t("[989,null,10250,98693,-89388,null,null,null,-32127]", 2);
        t("[-100,-200,-300,-20,-5,-10,null]", 3);
        t("[1,1,0,7,-8,-7,9]", 1);
    }
}
