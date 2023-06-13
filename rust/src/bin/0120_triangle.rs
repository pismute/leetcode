fn main() {
    assert_eq!(
        minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
        11
    );
    assert_eq!(minimum_total(vec![vec![-10]]), -10);
}

/*
 *    2
 *   3 4
 *  6 5 7
 * 4 1 8 3
 *
 * brute force
 * n = triangle.len()
 * O(2^n), O(n)
 *
 * dp: bottom up
 * dp[j] = triangle[i][j] + min(dp[j], dp[j+1])
 * i
 * 3: 4   1   8   3
 * 2: 7   6  10   _
 * 1: 9  10   _   _
 * 0: 11  _   _   _
 *
 * O(n*n), O(n)
 *
 * dp: top down
 * dp[j] = triangle[i][j] + min(dp[j], dp[j-1])
 * i
 * 0  2
 * 1  5   6
 * 2  11 10 13
 * 3  14 11 18 16
 *
 * O(n*n), O(n + 1)
 */
pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    let mut dp = triangle[triangle.len() - 1].clone();

    for i in (0..triangle.len() - 1).rev() {
        for j in 0..triangle[i].len() {
            dp[j] = triangle[i][j] + dp[j].min(dp[j + 1]);
        }
    }

    dp[0]
}
