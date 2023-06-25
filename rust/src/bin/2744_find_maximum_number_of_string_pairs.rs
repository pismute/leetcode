fn main() {
    assert_eq!(
        maximum_number_of_string_pairs(vec!["aa".to_string(), "ab".to_string()]),
        0
    );
    assert_eq!(
        maximum_number_of_string_pairs(vec!["ab".to_string(), "ba".to_string(), "cc".to_string()]),
        1
    );
    assert_eq!(
        maximum_number_of_string_pairs(vec![
            "cd".to_string(),
            "ac".to_string(),
            "dc".to_string(),
            "ca".to_string(),
            "zz".to_string()
        ]),
        2
    );
}

/*
 *
 *
 *
 *
 *
 *
 *
 */
pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
    type Set = std::collections::HashSet<String>;
    let mut dp: Set = Set::with_capacity(words.len());
    let mut res = 0;

    for s in words {
        if dp.contains(&s) {
            res += 1;
            dp.remove(&s);
        } else {
            dp.insert(s.chars().rev().collect());
        }
    }

    res
}
