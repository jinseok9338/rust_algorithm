pub fn int_to_roman(num: i32) -> String {
    let roman_map = [
        ("I", 1),
        ("IV", 4),
        ("V", 5),
        ("IX", 9),
        ("X", 10),
        ("XL", 40),
        ("L", 50),
        ("XC", 90),
        ("C", 100),
        ("CD", 400),
        ("D", 500),
        ("CM", 900),
        ("M", 1000),
    ];
    let mut roman = String::new();
    let mut num = num;

    for i in (0..roman_map.len()).rev() {
        while num >= roman_map[i].1 {
            num -= roman_map[i].1;
            roman += roman_map[i].0;
        }
    }
    roman
}

#[cfg(test)]
mod tests {
    use super::*;
    // Should return 'I' when given 1 as input
    #[test]
    fn test_return_I_when_given_1() {
        assert_eq!(int_to_roman(1), "I");
    }

    // Should return 'IV' when given 4 as input
    #[test]
    fn test_return_IV_when_given_4() {
        assert_eq!(int_to_roman(4), "IV");
    }

    // Should return an error when given a negative number as input
    #[test]
    fn test_return_empty_string_when_given_negative_number() {
        assert_eq!(int_to_roman(-1), "");
    }

    // Should return 'IX' when given 9 as input
    #[test]
    fn test_return_IX_when_given_9() {
        assert_eq!(int_to_roman(9), "IX");
    }
}
