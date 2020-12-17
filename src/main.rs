fn main() {
    println!("Reverse: {:?} to {}", 123, reverse_integer(123));
    let v = two_sum([3, 2, 3].to_vec(), 6);
    println!("Two sum: {:?}", v);

    test_next_larger_nodes(vec![2, 7, 4, 3, 5], vec![7, 0, 5, 5, 0]);
    test_next_larger_nodes(vec![1, 7, 5, 1, 9, 2, 5, 1], vec![7, 9, 9, 9, 0, 5, 0, 0]);
    test_next_larger_nodes(vec![2, 2, 5], vec![5, 5, 0]);
    test_next_larger_nodes(vec![9, 7, 6, 7, 6, 9], vec![0, 9, 7, 9, 9, 0]);
    test_next_larger_nodes(vec![4, 3, 2, 5, 1, 8, 10], vec![5, 5, 5, 8, 8, 10, 0]);
    
    println!("{}", num_of_subarrays(vec![1,3,5]));
    println!("{}", num_of_subarrays(vec![1,2,3,4,5,6,7]));
}

fn test_next_larger_nodes(in_v: Vec<i32>, out_v: Vec<i32>) {
    println!("{:?}", in_v);
    let res = next_larger_nodes(create_list(in_v));
    //assert!(out_v == res);
    println!("{:?}", res);
    println!("{:?}", out_v);
    println!("---------");
}

fn create_list(v: Vec<i32>) -> Option<Box<ListNode>> {
    let mut n: Box<ListNode> = Box::new(ListNode::new(v[v.len() - 1]));
    for i in 0..v.len() - 1 {
        let mut c = Box::new(ListNode::new(v[v.len() - 2 - i]));
        c.next = Some(n);
        n = c;
    }
    Some(n)
}

fn reverse_integer(mut x: i32) -> i32 {
    let mut res: i32 = 0;
    while x != 0 {
        res = res * 10 + x % 10;
        if res != 0 && res != res * 10 / 10 {
            return 0;
        }
        x /= 10;
    }
    res
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();
    for i in 0..nums.len() {
        for k in (i + 1)..nums.len() {
            let sum = nums[i] + nums[k];
            if sum == target {
                res.push(i as i32);
                res.push(k as i32);
                return res;
            }
        }
    }
    res
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
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

fn num_of_subarrays(arr: Vec<i32>) -> i32 {
    let mut prev = (arr[0] & 1, !arr[0] & 1);
    let mut res = prev.0;
    for i in arr.iter().skip(1) {
        let mut cur = (0, 0);
        if i & 1 == 1 {
            cur.0 = prev.1 + 1;
            cur.1 = prev.0;
        } else {
            cur.0 = prev.0;
            cur.1 = prev.1 + 1;
        }
        prev = cur;
        res = (res + prev.0) % 1000000007;
    }
    res
}
