fn main() {
    assert_eq!(
        word_break(
            "leetcode".to_string(),
            vec!["leet".to_string(), "code".to_string()]
        ),
        true
    );
    assert_eq!(
        word_break(
            "applepenapple".to_string(),
            vec!["apple".to_string(), "pen".to_string()]
        ),
        true
    );
    assert_eq!(
        word_break(
            "catsandog".to_string(),
            vec![
                "cats".to_string(),
                "dog".to_string(),
                "sand".to_string(),
                "and".to_owned(),
                "cat".to_owned()
            ]
        ),
        false
    );
}

/*
| "leetcode", ["leet","code"]
| [tffftffft]
|                      i=0;
|            "leet";i=4      "code"
|         "leet";i=8   "code"
|
| O(n*m*n)
|
| a[i] = a[i + w.len()
|
|
 */
pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let mut a = vec![false; s.len() + 1];
    a[s.len()] = true;

    let ss = s.as_bytes();
    for i in (0..s.len()).rev() {
        for word in &word_dict {
            let wb = word.as_bytes();
            if (i + wb.len()) <= ss.len() && wb == &ss[i..(i + wb.len())] {
                a[i] = a[i + wb.len()];

                if a[i] {
                    break;
                }
            }
        }
    }

    a[0]
}
