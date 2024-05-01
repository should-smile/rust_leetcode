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
