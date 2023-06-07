struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let m = nums1.len();
        let n = nums2.len();
        if (m + n) % 2 == 0 {
            (help(&nums1, &nums2, (m + n) / 2 - 1) as f64
                + help(&nums1, &nums2, (m + n) / 2) as f64)
                / 2.0
        } else {
            help(&nums1, &nums2, (m + n) / 2) as f64
        }
    }
}
pub fn help(nums1: &[i32], nums2: &[i32], k: usize) -> i32 {
    if nums1.len() > nums2.len() {
        return help(nums2, nums1, k);
    }
    if nums1.len() == 0 {
        return nums2[k];
    }
    if k == 0 {
        return nums1[0].min(nums2[0]);
    }

    if nums1[0] < nums2[0] {
        return help(&nums1[1..], nums2, k - 1);
    } else {
        return help(nums1, &nums2[1..], k - 1);
    }
}

mod tests {

    use super::*;
    #[test]
    fn test() {
        let s = Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]);
        assert_eq!(2.5, s, "1");
        let s = Solution::find_median_sorted_arrays(vec![1, 2, 3], vec![3, 4]);
        assert_eq!(3.0, s, "2");
    }
}
