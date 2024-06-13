use crate::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        let mut k = 0;
        for i in 0..nums.len() {
            if !set.contains(&nums[i]) {
                set.insert(nums[i]);
                nums[k] = nums[i];
                k += 1;
            }
        }
        k as i32
    }
}