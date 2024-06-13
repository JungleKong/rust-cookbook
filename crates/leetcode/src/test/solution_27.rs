use crate::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        let mut j = 0;
        while j < nums.len() {
            if nums[j] != val {
                nums[i] = nums[j];
                i += 1;
            }
            j += 1;
        }
        i as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_27() {
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;
        assert_eq!(Solution::remove_element(&mut nums, val), 2);
        assert_eq!(nums, vec![2, 2, 2, 3]);
    }
}