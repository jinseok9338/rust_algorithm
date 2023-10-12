pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (m, n) = (nums1.len(), nums2.len());

    if m > n {
        return find_median_sorted_arrays(nums2, nums1); // Ensure nums1 is the smaller array.
    }

    let (mut left, mut right) = (0, m);
    let half_len = (m + n + 1) / 2;

    while left <= right {
        let partition_x = (left + right) / 2;
        let partition_y = half_len - partition_x;

        let max_left_x = if partition_x == 0 {
            std::i32::MIN
        } else {
            nums1[partition_x - 1]
        };

        let min_right_x = if partition_x == m {
            std::i32::MAX
        } else {
            nums1[partition_x]
        };

        let max_left_y = if partition_y == 0 {
            std::i32::MIN
        } else {
            nums2[partition_y - 1]
        };

        let min_right_y = if partition_y == n {
            std::i32::MAX
        } else {
            nums2[partition_y]
        };

        if max_left_x <= min_right_y && max_left_y <= min_right_x {
            if (m + n) % 2 == 0 {
                return (f64::from(std::cmp::max(max_left_x, max_left_y))
                    + f64::from(std::cmp::min(min_right_x, min_right_y)))
                    / 2.0;
            } else {
                return f64::from(std::cmp::max(max_left_x, max_left_y));
            }
        } else if max_left_x > min_right_y {
            right = partition_x - 1;
        } else {
            left = partition_x + 1;
        }
    }

    // If no solution is found, return an error or some default value.
    0.0
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]

    fn test_find_median_sorted_arrays() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.0);

        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.5);

        let nums1 = vec![0, 0];
        let nums2 = vec![0, 0];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 0.0);

        let nums1 = vec![];
        let nums2 = vec![1];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 1.0);

        let nums1 = vec![2];
        let nums2 = vec![];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.0);
    }
}
