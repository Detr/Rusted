use crate::utils::structures::ListNode;

pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
    _next_larger_nodes(&head)
}

fn _next_larger_nodes(head: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut v: Vec<i32>;

    match &head {
        Some(curr) => {
            v = _next_larger_nodes(&curr.next);
            match &curr.next {
                Some(next) => {
                    if curr.val < next.val {
                        v.insert(0, next.val);
                    } else {
                        v.insert(0, 0);
                        for i in v.iter() {
                            if curr.val < *i {
                                v[0] = *i;
                                break;
                            }
                        }
                    }
                }
                None => {}
            }
        }
        None => {
            v = vec![0];
        }
    }
    v
}