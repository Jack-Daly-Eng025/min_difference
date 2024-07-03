pub struct Solution;
impl Solution {
    pub fn min_difference(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        if n <= 3 {
            return 0
        }
        let mut r = i32::MAX;
        for i in 0..4 {
            r = r.min(nums[n - i - 1] - nums[3 - i]);
        }
        r
    }
}
fn main() {
    let min_dif = Solution::min_difference(vec![100,3,20,50]);
    println!("{min_dif}");
}
