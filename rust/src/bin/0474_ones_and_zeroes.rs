fn main() {
    assert_eq!(
        find_max_form(vec!["10".to_owned(), "0".to_owned(), "1".to_owned()], 1, 1),
        2
    );
    assert_eq!(
        find_max_form(
            vec![
                "10".to_owned(),
                "0001".to_owned(),
                "111001".to_owned(),
                "1".to_owned(),
                "0".to_owned()
            ],
            4,
            3
        ),
        3
    );
    assert_eq!(
        find_max_form(
            vec![
                "10".to_owned(),
                "0001".to_owned(),
                "111001".to_owned(),
                "1".to_owned(),
                "0".to_owned()
            ],
            5,
            3
        ),
        4
    );
    assert_eq!(
        find_max_form(
            vec![
                "0".to_owned(),
                "11".to_owned(),
                "1000".to_owned(),
                "01".to_owned(),
                "0".to_owned(),
                "101".to_owned(),
                "1".to_owned(),
                "1".to_owned(),
                "1".to_owned(),
                "0".to_owned(),
                "0".to_owned(),
                "0".to_owned(),
                "0".to_owned(),
                "1".to_owned(),
                "0".to_owned(),
                "0110101".to_owned(),
                "0".to_owned(),
                "11".to_owned(),
                "01".to_owned(),
                "00".to_owned(),
                "01111".to_owned(),
                "0011".to_owned(),
                "1".to_owned(),
                "1000".to_owned(),
                "0".to_owned(),
                "11101".to_owned(),
                "1".to_owned(),
                "0".to_owned(),
                "10".to_owned(),
                "0111".to_owned()
            ],
            9,
            80
        ),
        17
    );
    assert_eq!(
        find_max_form(
            vec![
                "011".to_owned(),
                "1".to_owned(),
                "11".to_owned(),
                "0".to_owned(),
                "010".to_owned(),
                "1".to_owned(),
                "10".to_owned(),
                "1".to_owned(),
                "1".to_owned(),
                "0".to_owned(),
                "0".to_owned(),
                "0".to_owned(),
                "01111".to_owned(),
                "011".to_owned(),
                "11".to_owned(),
                "00".to_owned(),
                "11".to_owned(),
                "10".to_owned(),
                "1".to_owned(),
                "0".to_owned(),
                "0".to_owned(),
                "0".to_owned(),
                "0".to_owned(),
                "101".to_owned(),
                "001110".to_owned(),
                "1".to_owned(),
                "0".to_owned(),
                "1".to_owned(),
                "0".to_owned(),
                "0".to_owned(),
                "10".to_owned(),
                "00100".to_owned(),
                "0".to_owned(),
                "10".to_owned(),
                "1".to_owned(),
                "1".to_owned(),
                "1".to_owned(),
                "011".to_owned(),
                "11".to_owned(),
                "11".to_owned(),
                "10".to_owned(),
                "10".to_owned(),
                "0000".to_owned(),
                "01".to_owned(),
                "1".to_owned(),
                "10".to_owned(),
                "0".to_owned()
            ],
            44,
            39
        ),
        45
    );
}

/*
 * [10, 0, 1] 1, 1
 *
 *                                      (m=1, n=1, i=0)
 *                        10;(0, 0, 0);O           0;(0, 1, 1)                   1;(1, 0, 2)
 *                 0;i=1;X        1;i=2;X          1;(0, 0, 2);O
 *               1;i=2
 *
 *
 *
 * brute force:
 * O(2^n), O(n)
 *
 * Cache: dp
 *
 * [10, 0001, 111001, 1, 0], m = 4, n= 3
 *
 * "10" , mm = 1, nn = 1, j in m..=mm, k in n..=nn
 * m\n 0 1 2 3
 * 0   0 0 0 0
 * 1   0 1 1 1
 * 2   0 1 1 1
 * 3   0 1 1 1
 * 4   0 1 1 1
 *
 * "0001" , mm = 3, nn = 1, j in m..=mm, k in n..=nn
 * m\n 0 1 2 3
 * 0   0 0 0 0
 * 1   0 1 1 1
 * 2   0 1 1 1
 * 3   0 1 1 1
 * 4   0 1 2 2
 *
 * "111001", mm = 2, nn = 4, j in m..=mm, k in n..=nn
 * m\n 0 1 2 3
 * 0   0 0 0 0
 * 1   0 1 1 1
 * 2   0 1 1 1
 * 3   0 1 1 1
 * 4   0 1 2 2
 *
 * "1", mm = 0, nn = 1, j in m..=mm, k in n..=nn
 * m\n 0 1 2 3
 * 0   0 1 1 1
 * 1   0 1 2 2
 * 2   0 1 2 2
 * 3   0 1 2 2
 * 4   0 1 2 3
 *
 * "0", mm = 1, nn = 0, j in m..=mm, k in n..=nn
 * m\n 0 1 2 3
 * 0   0 1 1 1
 * 1   1 2 2 2
 * 2   1 2 3 3
 * 3   1 2 3 3
 * 4   1 2 3 3
 *
 * dp[j][k] = max(dp[j][k], 1 + dp[j - mm][k - nn])
 *
 * O(m*n*len(strs)), O(m*n)
 */
pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
    fn to_nrs(s: String) -> (usize, usize) {
        s.chars().fold((0, 0), |acc, x| {
            if x == '0' {
                (acc.0 + 1, acc.1)
            } else {
                (acc.0, acc.1 + 1)
            }
        })
    }

    let mut dp = vec![vec![0; n as usize + 1]; m as usize + 1];

    for (mm, nn) in strs.into_iter().map(to_nrs) {
        for j in (mm..=m as usize).rev() {
            for k in (nn..=n as usize).rev() {
                dp[j][k] = dp[j][k].max(1 + dp[j - mm][k - nn]);
            }
        }
    }

    dp[m as usize][n as usize]
}
