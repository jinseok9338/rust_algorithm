pub fn min_sub_array(target: i32, nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut sum = 0;
    let mut res = i32::MAX;

    for right in 0..nums.len() {
        sum += nums[right];
        while sum >= target {
            res = res.min(right as i32 - left as i32 + 1);
            sum -= nums[left];
            left += 1;
        }
    }

    if res == i32::MAX {
        res = 0;
    }

    return res;
}

// Returns the minimum length of a contiguous subarray of which the sum is greater than or equal to the target.
#[test]
fn test_min_sub_array_sum_greater_than_target() {
    let target = 5;
    let nums = vec![1, 2, 3, 4, 5];
    assert_eq!(min_sub_array(target, nums), 1);
}
