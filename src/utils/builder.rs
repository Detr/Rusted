use crate::utils::structures::ListNode;

pub fn create_list(v: Vec<i32>) -> Option<Box<ListNode>> {
    let mut n: Box<ListNode> = Box::new(ListNode::new(v[v.len() - 1]));
    for i in 0..v.len() - 1 {
        let mut c = Box::new(ListNode::new(v[v.len() - 2 - i]));
        c.next = Some(n);
        n = c;
    }
    Some(n)
}