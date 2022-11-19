use leet_code_rust::longest_substring_without_repeating_characters::solution;

#[test]
pub fn solution_can_be_accepted() {
    assert_eq!(solution::length_of_longest_substring(String::from("abcabcbb")), 3);
    assert_eq!(solution::length_of_longest_substring(String::from("bbbbb")), 1);
    assert_eq!(solution::length_of_longest_substring(String::from("pwwkew")), 3);
    assert_eq!(solution::length_of_longest_substring(String::from("abab")), 2);
    assert_eq!(solution::length_of_longest_substring(String::from("")), 0);
    assert_eq!(solution::length_of_longest_substring(String::from("a")), 1);
}
