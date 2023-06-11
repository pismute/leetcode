fn main() {
    assert_eq!(longest_palindrome("babad".to_string()), "bab");
    assert_eq!(longest_palindrome("cbbd".to_string()), "bb");
}

/*
 *
 * "babad"
 *
 * brute force:
 * is_palindrome: O(n)
 *  i ->
 * "babad"
 *   <- j
 * all substrings: O(n^2)
 *  i ->
 * "babad"
 *  j ->
 *
 * O(n^3), O(1)
 *
 * determining it from center.
 * moving center: n
 * find a palindrome: n
 *
 * O(n^2), O(1)
 *
 */
pub fn longest_palindrome(s: String) -> String {
    let mut start = 0;
    let mut end = 0;

    let ss = s.as_bytes();
    for center in 0..ss.len() {
        // for odd length
        for i in 0..=center.min(ss.len() - 1 - center) {
            let j = center - i;
            let k = center + i;

            if ss[j] != ss[k] {
                break;
            } else if k - j > end - start {
                start = j;
                end = k;
            }
        }

        // for even length
        for i in 0..=center.min(ss.len() - 1 - center) {
            let j = center - i;
            let k = center + i + 1;

            if k >= ss.len() || ss[j] != ss[k] {
                break;
            } else if k - j > end - start {
                start = j;
                end = k;
            }
        }
    }

    String::from_utf8(ss[start..=end].to_owned()).unwrap()
}
