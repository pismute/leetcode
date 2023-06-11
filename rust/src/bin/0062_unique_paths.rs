fn main() {
    assert_eq!(unique_paths(3, 7), 28);
    assert_eq!(unique_paths(3, 2), 3);
}

/*
 * m = 3, n = 2
 *
 *                             (0, 0)
 *                        (1, 0)       (0, 1)
 *                    (2, 0) (1, 1)  (1,1)  (0,2)
 *                           .........
 *
 * brute force
 * O(2^(m+n)), O(m+n)
 *
 * dp: top down
 *
 * 2-dimensional
 *   0 1
 * 0 1 1
 * 1 1 2
 * 2 1 3
 *
 * a[i][j] = a[i-1][j] + a[i][j-1]
 *
 * O(m*n), O(m*n)
 *
 * 1-dimension
 *
 * a[i][j] = a[i][j] + a[i][j-1]
 *
 * O(m*n), O(n)
 */
pub fn unique_paths(m: i32, n: i32) -> i32 {
    let mut a = vec![0; n as usize + 1];

    a[1] = 1;
    for _ in 0..m {
        for j in 1..=n as usize {
            a[j] += a[j - 1];
        }
    }

    a[n as usize]
}
