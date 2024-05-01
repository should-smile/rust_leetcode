use rust_leetcode::solution::commen_def::ListNode;
fn main() {
    let list = ListNode::from_vec(vec![1, 2, 3]);
    println!("{:?}", ListNode::to_vec(&list));
}
