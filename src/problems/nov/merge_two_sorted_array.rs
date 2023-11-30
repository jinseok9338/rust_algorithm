pub fn merge_two_sorted_array(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut last = (m + n) - 1;
    let mut m = m;
    let mut n = n;
    while m > 0 && n > 0 {
        if nums1[m as usize - 1] > nums2[n as usize - 1] {
            nums1[last as usize] = nums1[m as usize - 1];
            m -= 1;
        } else {
            nums1[last as usize] = nums2[n as usize - 1];
            n -= 1;
        }
        last -= 1;
    }
    while n > 0 {
        nums1[last as usize] = nums2[n as usize - 1];
        n -= 1;
        last -= 1;
    }
}

#[test]
fn test() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    merge_two_sorted_array(&mut nums1, 3, &mut nums2, 3);
    assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
}
