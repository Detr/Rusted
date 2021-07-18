pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
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