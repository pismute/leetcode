fn main() {
    assert_eq!(longest_palindrome_subseq("abaabaa".to_string()), 6);
    assert_eq!(longest_palindrome_subseq("bbbab".to_string()), 4);
    assert_eq!(longest_palindrome_subseq("cbbd".to_string()), 2);
}

/*
 * bbbab
 * 22222 = 2^n
 *
 * isPalindrome: n
 *
 * brute force
 * O(2^n * n), O(n)
 *
 * dp: bottom up
 * dp[i][j] = if s[i] == r[j] { d[i+1][j+1] + 1} else { dp[i+1][j].max(dp[i][j+1] }
 *
 *  b b b a b
 *b 4 3 3 2 1
 *a 3 3 2 2 1
 *b 3 3 2 1 1
 *b 2 2 2 1 1
 *b 1 1 1 1 1
 *
 * result = dp[0][0] = 4
 *
 * O(n * n), O(n*n)
 *
 * reuse array
 *
 * O(n * n), O(n * 2)
 *
 * dp: top down
 * dp[i][j] = if s[i] == r[j] { d[i-1][j-1] + 1} else { dp[i-1][j].max(dp[i][j-1] }
 *  b b b a b
 *b 1 1 1 1 1
 *a 1 1 1 2 1
 *b 1 2 2 2 3
 *b 1 2 3 3 3
 *b 1 2 3 3 4
 *
 * result = dp[last][last] = 4
 *
 * O(n*n), O(n*n)
 *
 * reuse array
 *
 * O(n * n), O(n * 2)
 *
 */
pub fn longest_palindrome_subseq(s: String) -> i32 {
    let ss: &[u8] = s.as_bytes();
    let len = ss.len();
    let mut dp = vec![vec![0; ss.len() + 1]; 2];

    for i in 1..=len {
        for j in 1..=len {
            if ss[len - i] == ss[j - 1] {
                dp[i % 2][j] = dp[(i - 1) % 2][j - 1] + 1;
            } else {
                dp[i % 2][j] = dp[(i - 1) % 2][j].max(dp[i % 2][j - 1]);
            }
        }
    }

    dp[len % 2][len]
}
