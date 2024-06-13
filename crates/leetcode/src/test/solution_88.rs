use crate::Solution;


impl Solution {
    #[allow(dead_code)]
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut m = m as usize;
        let mut n = n as usize;
        let mut i = m + n - 1;
        while m > 0 && n > 0 {
            if nums1[m - 1] > nums2[n - 1] {
                nums1[i] = nums1[m - 1];
                m -= 1;
            } else {
                nums1[i] = nums2[n - 1];
                n -= 1;
            }
            i -= 1;
        }
        while n > 0 {
            nums1[i] = nums2[n - 1];
            n -= 1;
            i -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_88() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }
}