use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut h = HashMap::new();
        for (i, v) in nums.iter().enumerate() {
            h.insert(v, i);
        }

        let mut res = Vec::new();
        for (i, v) in nums.iter().enumerate() {
            let k = target - v;
            match h.get(&k) {
                Some(&pos) => {
                    if pos != i {
                        res.push(i as i32);
                        res.push(pos as i32);
                        break;
                    }
                }
                _ => {}
            }
        }
        return res;
    }
}