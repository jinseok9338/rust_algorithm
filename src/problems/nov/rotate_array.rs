pub fn rotate_array(nums: &mut [i32], k: i32) {
    let k = k % nums.len() as i32;
    nums.reverse();
    nums[..k as usize].reverse();
    nums[k as usize..].reverse();
}

#[test]
fn test_rotate_array() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    rotate_array(&mut nums, 3);
    assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
}

#[test]
fn test_rotate_array_2() {
    let mut nums = vec![-1, -100, 3, 99];
    rotate_array(&mut nums, 2);
    assert_eq!(nums, vec![3, 99, -1, -100]);
}
