fn main() {
    assert_eq!(change(5, vec![1, 2, 5]), 4);
    assert_eq!(change(3, vec![2]), 0);
    assert_eq!(change(10, vec![10]), 1);
}
/*
 * 5, [1,2,5]
 *                                      0;5
 *                  1;4                      2;3            5;0
 *         1:3              2:2     ,   1:2      2:1
 *     1:2       2:1 ,  1:1    2:0  , 1:1 2;0 ,  1:0
 *   1:1  2:0  1:0   ,  1:0    1:0
 *   1:0
 *
 * brute force
 * O(c^n), O(n*n)
 * Set[subrray]
 *
 * brute force2 - once a coin is chosen, smaller coins cannot be chosen.
 *                                      0;5
 *                  1;4                      2;3            5;0
 *         1:3              2:2     ,        2:1
 *     1:2        ,         2:0     ,
 *   1:1  2:0     ,
 *   1:0
 *
 * O(c^n), O(n)
 *
 * dp: bottom up
 *
 * array[coins][amount] =
 *   0 1 2 3 4 5 - index
 *   5 4 3 2 1 0
 * 1 4 3 2 2 1 1
 * 2 1 1 0 1 0 1
 * 5 1 0 0 0 0 1
 *   --
 * O(c*n), O(c*n)
 *
 * dp2
 * O(c*n), O(n)
 *
 * dp: topdown
 *
 * array[coins][amount] =
 *   0 1 2 3 4 5 - index
 *   0 1 2 3 4 5
 * 1 1 1 1 1 1 1
 * 2 1 1 2 2 3 3
 * 5 1 1 2 2 2 4
 *   --
 * O(c*n), O(c*n)
 *
 * dp2
 * O(c*n), O(n)
 *
 *   --
 */
pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
    let mut dp = vec![0; amount as usize + 1];

    dp[0] = 1;

    for c in coins {
        for a in 1..=amount as usize {
            if a >= c as usize {
                dp[a] += dp[a - c as usize];
            }
        }
    }

    dp[amount as usize]
}
