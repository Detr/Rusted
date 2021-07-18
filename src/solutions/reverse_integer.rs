pub fn reverse_integer(mut x: i32) -> i32 {
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