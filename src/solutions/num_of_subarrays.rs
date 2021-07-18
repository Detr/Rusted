pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
    let mut prev: (i32, i32) = (arr[0] & 1, !arr[0] & 1);
    let mut res: i32 = prev.0;
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
