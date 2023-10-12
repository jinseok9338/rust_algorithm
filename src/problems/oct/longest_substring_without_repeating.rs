//https://leetcode.com/problems/longest-substring-without-repeating-characters/
use std::collections::HashMap;
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut start = 0;
    let mut maxlen = 0;
    let mut map: HashMap<char, usize> = HashMap::new();

    for (i, c) in s.chars().enumerate() {
        if map.contains_key(&c) {
            start = std::cmp::max(*map.get(&c).unwrap() + 1, start);
        }
        maxlen = std::cmp::max(maxlen, i - start + 1);
        map.insert(c, i);
    }
    maxlen as i32
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
        assert_eq!(length_of_longest_substring("".to_string()), 0);
    }
}
