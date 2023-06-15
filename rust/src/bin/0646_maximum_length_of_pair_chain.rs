fn main() {
    assert_eq!(
        find_longest_chain(vec![vec![3, 4], vec![2, 3], vec![1, 2]]),
        2
    );
    assert_eq!(
        find_longest_chain(vec![vec![1, 2], vec![2, 3], vec![3, 4]]),
        2
    );
    assert_eq!(
        find_longest_chain(vec![vec![1, 2], vec![7, 8], vec![4, 5]]),
        3
    );
    assert_eq!(
        find_longest_chain(vec![vec![1, 2], vec![7, 8], vec![4, 5]]),
        3
    );
    assert_eq!(
        find_longest_chain(vec![
            vec![-10, -8],
            vec![8, 9],
            vec![-5, 0],
            vec![6, 10],
            vec![-6, -4],
            vec![1, 7],
            vec![9, 10],
            vec![-4, 7]
        ]),
        4
    );
}

/*
 * Longest increasing subsequence.
 * [[1, 2], [2, 3], [3, 4]]
 *    2       2        2    = 2^n
 *
 * brute force
 * O(2^n), O(n)
 *
 * dp: bottom up
 *
 * dp[i] = 1 + dp[i].max(1 + dp[j])
 *
 * [[1, 2], [2, 3], [3, 4]]
 * [   2   ,   1    ,   1 ]
 *
 * sort: nlogn
 * LIS: n^2
 * O(nlogn + n^2), O(n)
 *
 *
 * LIS: n^2
 * O(nlogn + n^2), O(n)
 *
 * dp: top down
 *
 *
 * [[1, 2], [4, 5], [7, 8]] // sorted
 * [  1  ,   2   ,   3 ]
 *
 * O(nlogn + n^2), O(n)
 *
*/
pub fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
    let mut sorted = pairs.clone();

    sorted.sort_by_key(|v| v[0] + v[1]);

    let mut dp: Vec<i32> = vec![0; pairs.len()];

    let mut max = 0;

    for i in 0..sorted.len() {
        for j in 0..=i {
            if sorted[i][0] > sorted[j][1] {
                dp[i] = dp[i].max(dp[j] + 1);
            } else {
                dp[i] = dp[i].max(1);
            }
        }
        max = max.max(dp[i]);
    }

    max
}
