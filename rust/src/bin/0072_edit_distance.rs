fn main() {
    assert_eq!(min_distance("horse".to_string(), "ros".to_string()), 3);
    assert_eq!(
        min_distance("intention".to_string(), "execution".to_string()),
        5
    );
}

/*
 * LCS
 *
 *   " r o s
 * " 0 1 2 3
 * h 1 1 2 3
 * o 2 2 1 2
 * r 3 2 2 2
 * s 4 3 3 2
 * e 5 4 4 3
 *
 *   i
 *  horse
 *  raos
 *   j
 *
 * w[i] == w[j] = (i+1, j+1)
 * insert: (i, j+1)
 * delete: (i+1, j)
 * replace: (i+1, j+1)
 * O(n*m), O(2m)
 *
 */
pub fn min_distance(word1: String, word2: String) -> i32 {
    let mut dp = vec![vec![0; word2.len() + 1]; 2];

    // base case
    for j in 1..=word2.len() {
        dp[1][j] = j;
    }

    let b1 = word1.as_bytes();
    let b2 = word2.as_bytes();
    for i in 0..word1.len() {
        dp[i % 2][0] = i + 1; // base case
        for j in 1..=word2.len() {
            if b1[i] == b2[j - 1] {
                dp[i % 2][j] = dp[(i + 1) % 2][j - 1]
            } else {
                dp[i % 2][j] = dp[(i + 1) % 2][j - 1]
                    .min(dp[i % 2][j - 1])
                    .min(dp[(i + 1) % 2][j])
                    + 1;
            }
        }
    }

    dp[(word1.len() + 1) % 2][word2.len()] as i32
}
