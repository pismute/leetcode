fn main() {
    assert_eq!(
        minimize_concatenated_length(vec!["aaa".to_string(), "bba".to_string(), "bb".to_string()]),
        6
    );
    assert_eq!(
        minimize_concatenated_length(vec!["aa".to_string(), "ab".to_string(), "bc".to_string()]),
        4
    );
    assert_eq!(
        minimize_concatenated_length(vec!["ab".to_string(), "b".to_string()]),
        2
    );
    assert_eq!(
        minimize_concatenated_length(vec!["aaa".to_string(), "c".to_string(), "aba".to_string()]),
        6
    );
}

/*
 *
 * ["aa","ab","dd"]
 *                                2
 *                "aab";3                                                           "abaa": 4
 *           "aabdd":5             "ddaab":5                           "abaadd": 6         "ddabaa":  6
 *
 * O(n*(26*26)), O(26*26)
 */
pub fn minimize_concatenated_length(words: Vec<String>) -> i32 {
    let wb = words.iter().map(|x| x.as_bytes()).collect::<Vec<_>>();
    type Map = std::collections::HashMap<(u8, u8), usize>;

    let mut dp = Map::new();
    dp.insert((wb[0][0], wb[0][wb[0].len() - 1]), wb[0].len());

    for i in 1..wb.len() {
        let mut next = Map::new();

        for ((s, e), len) in dp {
            let w = wb[i];
            let len1 = len
                + if s == w[w.len() - 1] {
                    w.len() - 1
                } else {
                    w.len()
                };

            next.entry((w[0], e))
                .and_modify(|x| *x = len1.min(*x))
                .or_insert(len1);

            let len2 = len + if e == w[0] { w.len() - 1 } else { w.len() };

            next.entry((s, w[w.len() - 1]))
                .and_modify(|x| *x = len2.min(*x))
                .or_insert(len2);
        }

        dp = next;
    }

    *dp.values().min().unwrap() as i32
}
