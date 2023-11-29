pub fn single_numer_two(nums: Vec<i32>) -> i32 {
    let mut ones = 0;
    let mut twos = 0;

    for num in nums {
        ones = ones ^ num & !twos;
        twos = twos ^ num & !ones;
    }
    ones
}


#[test]
fn test() {
    assert_eq!(single_numer_two(vec![2, 2, 3, 2]), 3);
    assert_eq!(single_numer_two(vec![0, 1, 0, 1, 0, 1, 99]), 99);
}