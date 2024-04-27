use std::collections::{HashMap, BinaryHeap};

impl Solution {
    pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
        struct VP(usize, i32);
        impl Eq for VP {
        }
        impl PartialEq for VP {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        impl PartialOrd for VP {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }
        impl Ord for VP {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.0.cmp(&other.0)
            }
        }

        let mut cnt_map: HashMap<i32, usize> = HashMap::new();
        for i in barcodes {
            let c = cnt_map.entry(i).or_insert(0);
            *c += 1;
        }

        let mut heap = BinaryHeap::new();
        for i in cnt_map {
            heap.push(VP(i.1, i.0));
        }

        let mut res = Vec::new();
        while heap.len() >= 2 {
            let mut a = heap.pop().unwrap();
            let mut b = heap.pop().unwrap();
            res.push(a.1);
            res.push(b.1);
            a.0 -= 1;
            b.0 -= 1;
            if a.0 > 0 {
                heap.push(a);
            }
            if b.0 > 0 {
                heap.push(b);
            }
        }

        while !heap.is_empty() {
            let a = heap.pop().unwrap();
            res.push(a.1);

        }
        return res;
    }
}

pub struct Solution;

#[cfg(test)]
mod t {
    use crate::solution::rearrange_barcodes::Solution;
    #[test]
    fn it_works() {
        fn t(barcodes: Vec<i32>) {
            let n = barcodes.len();
            let res = Solution::rearrange_barcodes(barcodes);

            assert_eq!(res.len(), n);

            println!("{:?}", res);
            for i in 1..res.len() {
                assert_ne!(res[i - 1], res[i])
            }
        }
        t(vec![1, 1, 1, 2, 2, 2]);
        t(vec![1, 1, 1, 1, 2, 2, 3, 3]);
        t(vec![1]);
    }
}
