mod utils;
mod solutions;

use crate::utils::builder::create_list;

use crate::solutions::next_larger_nodes::*;
use crate::solutions::reverse_integer::*;
use crate::solutions::num_of_subarrays::*;
use crate::solutions::two_sum::*;

fn main() {
    println!("Reverse: {:?} to {}", 123, reverse_integer(123));
    let v = two_sum([3, 2, 3].to_vec(), 6);
    println!(r#"Two sum: {:?}"#, v);

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