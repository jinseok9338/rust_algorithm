pub fn candy(ratings: Vec<i32>) -> i32 {
    let mut candies = vec![1; ratings.len()];
    for i in 1..ratings.len() {
        if ratings[i] > ratings[i - 1] {
            candies[i] = candies[i - 1] + 1;
        }
    }
    for i in (0..ratings.len() - 1).rev() {
        if ratings[i] > ratings[i + 1] {
            candies[i] = std::cmp::max(candies[i], candies[i + 1] + 1);
        }
    }
    candies.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_simple_input() {
        let ratings = vec![1, 2, 3, 4, 5];
        let result = candy(ratings);
        assert_eq!(result, 15);
    }
    // Calculates the correct number of candies for an input with decreasing ratings
    #[test]
    fn test_decreasing_ratings() {
        let ratings = vec![5, 4, 3, 2, 1];
        let result = candy(ratings);
        assert_eq!(result, 15);
    }

    #[test]
    #[should_panic]
    fn test_empty_input() {
        let ratings = vec![];
        candy(ratings);
    }
    #[test]
    fn test_negative_ratings() {
        let ratings = vec![-1, -2, -3, -4, -5];
        let result = candy(ratings);
        assert_eq!(result, 15);
    }
}
