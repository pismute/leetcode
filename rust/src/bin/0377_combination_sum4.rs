fn main() {
    assert_eq!(combination_sum4(vec![1, 2, 3], 4), 7);
    assert_eq!(combination_sum4(vec![9], 3), 0);
}

/*
 * [1, 2, 3], 4
 *
 *                1                           2                                    3
 *        1       2          3       1        2          3
 *               ......................
 * brute force:
 * O(c^n), O(n)
 *
 * dp: bottom up
 *
 * dp[0] = 1
 * dp[1] = dp[1 - 1] = 1
 * dp[2] = dp[2 - 1] + dp[2 - 2] = 2
 * dp[3] = dp[3 - 1] + dp[3 - 2]  + dp[3 - 3] = 4
 * dp[4] = dp[4 - 1] + dp[4 - 2]  + dp[4 - 3] = 7
 *
 * O(c*n), O(n)
 *
 * reuse space
 * O(c*n), O(max(nums))
 */
pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
    let mut dp = vec![0; target as usize + 1];

    dp[0] = 1;
    for i in 0..=target as usize {
        for n in &nums {
            if i >= *n as usize {
                dp[i] += dp[i - *n as usize];
            }
        }
    }

    dp[target as usize]
}
