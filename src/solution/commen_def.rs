use regex::Regex;
use std::cell::RefCell;
use std::collections::btree_map::VacantEntry;
use std::collections::VecDeque;
use std::rc::Rc;
use std::{fmt::Display, str::FromStr};

#[macro_export]
macro_rules! vecnd {
    ($([$($inner:tt)*]),+ $(,)?) => {
        vec![$(
            vecnd![$($inner)*]
        ),+]
    };
    ($($t:tt)*) => {
        vec![$($t)*]
    };
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from_vec(vals: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut tail = &mut head;

        for num in vals {
            *tail = Some(Box::new(Self::new(num)));
            tail = &mut tail.as_mut().unwrap().next;
        }

        return head;
    }

    pub fn to_vec(list: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec = Vec::new();

        let mut head = list;
        while let Some(v) = head {
            vec.push(v.val);
            head = &v.next;
        }

        return vec;
    }
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    #[inline]
    fn new_rc(v: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::new(v)))
    }

    /**
     * 将二叉树层序数组转化为二叉树
     */
    pub fn level_order_to_tree(vals: &Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() {
            return None;
        }

        let root = Self::new_rc(vals[0].unwrap());
        let mut queue = VecDeque::new();
        queue.push_back(Rc::clone(&root));

        let mut idx = 1;
        while !queue.is_empty() && idx < vals.len() {
            let node = queue.pop_front().unwrap();

            if let Some(val) = vals[idx] {
                let left_node = Self::new_rc(val);
                node.borrow_mut().left = Some(Rc::clone(&left_node));
                queue.push_back(left_node);
            }
            idx += 1;

            if idx >= vals.len() {
                break;
            }

            if let Some(val) = vals[idx] {
                let right_node = Self::new_rc(val);
                node.borrow_mut().right = Some(Rc::clone(&right_node));
                queue.push_back(right_node);
            }
            idx += 1;
        }

        return Some(root);
    }

    /**
     * `from_str` 将字符串形式的二叉树层序数组转化为二叉树
     * # Examples
     * ```
     * let root1 = from_str("[]");
     * let root2 = from_str(" [1,7,0,7,-8,null,null]");
     * let root3 = from_str("[989, null,10250,98693,-89388,null,null,null,-32127]");
     * ```
     */
    pub fn from_str(data: &str) -> Option<Rc<RefCell<TreeNode>>> {
        let vec = Self::str_vec_to_vec(data);
        return Self::level_order_to_tree(&vec);
    }

    pub fn str_vec_to_vec(data: &str) -> Vec<Option<i32>> {
        let re = Regex::new(r"-?\d+|null").unwrap();

        let mut vec = Vec::new();
        let mut idx = 0;
        while let Some(m) = re.find_at(data, idx) {
            vec.push(match m.as_str().parse() {
                Ok(v) => Some(v),
                Err(_) => None,
            });

            idx = m.end();
        }
        return vec;
    }
}

#[cfg(test)]
mod test {
    use crate::solution::commen_def::TreeNode;

    #[test]
    fn str_vec_to_vec() {
        fn t(data: &str, pre: Vec<Option<i32>>) {
            let res = TreeNode::str_vec_to_vec(data);
            assert_eq!(res, pre);
        }

        t("[]", vec![]);
        t(
            " [1,7,0,7,-8,null,null]",
            vec![Some(1), Some(7), Some(0), Some(7), Some(-8), None, None],
        );
        t(
            "[989, null,10250,98693,-89388,null,null,null,-32127]",
            vec![
                Some(989),
                None,
                Some(10250),
                Some(98693),
                Some(-89388),
                None,
                None,
                None,
                Some(-32127),
            ],
        );
    }
}
