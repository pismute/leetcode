fn main() {
    assert_eq!(count_good_strings(3, 3, 1, 1), 8);
    assert_eq!(count_good_strings(2, 3, 1, 2), 5);
    assert_eq!(count_good_strings(5, 5, 3, 1), 4);
    assert_eq!(count_good_strings(1, 100000, 1, 1), 215447031);
    assert_eq!(count_good_strings(200, 200, 10, 1), 764262396);
}

/*
 * 2, 3, 1, 2
 *
 *                             0                11
 *                          00   011         110 1111X
 *                         000
 * dp[len]
 * dp[0] = 1 // empty set.
 * dp[1] = dp[1 - 1] + dp[2-2] = 1 + 0 = 1
 * dp[2] = dp[2 - 1] + dp[2-2] = 1 + 1 = 2
 * dp[3] = dp[3 - 1] + dp[3-2] = 2 + 1 = 3
 *
 * O(high), O(min(max(zero, on), high))
 *
 *                      000                 1
 *                      0001           11                  1000
 *                      00011      111    11000       10001
 *                                1111
 *                                11111
 * dp[0] = 1
 * dp[1] = 1
 * dp[2] = dp[2 - 1] + dp[2-3] = 1
 * dp[3] = dp[3 - 1] + dp[3-3] = 2
 * dp[4] = dp[4 - 1] + dp[4-3] = 2 + 1 = 3
 * dp[5] = dp[5 - 1] + dp[5-3] = 3 + 1 = 4
 *
 */
pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
    const MOD: i32 = 10_i32.pow(9) + 7;
    let size = zero.max(one).min(high);
    let mut dp = vec![0; size as usize];

    dp[0] = 1;

    let mut sum = 0;
    for i in 1..=high {
        let i_zero = if i >= zero {
            dp[((i - zero) % size) as usize]
        } else {
            0
        };
        let i_one = if i >= one {
            dp[((i - one) % size) as usize]
        } else {
            0
        };

        let nr = (i_zero + i_one) % MOD;
        dp[(i % size) as usize] = nr;

        if i >= low && i <= high {
            sum = (sum + nr) % MOD;
        }
    }

    sum % MOD
}
