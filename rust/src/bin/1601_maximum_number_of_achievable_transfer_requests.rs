fn main() {
    assert_eq!(
        maximum_requests(3, vec![vec![0, 0], vec![1, 2], vec![2, 1]]),
        3
    );
    assert_eq!(
        maximum_requests(
            3,
            vec![
                vec![0, 0],
                vec![1, 1],
                vec![0, 0],
                vec![2, 0],
                vec![2, 2],
                vec![1, 1],
                vec![2, 1],
                vec![0, 1],
                vec![0, 1]
            ]
        ),
        5
    );
    assert_eq!(
        maximum_requests(
            5,
            vec![
                vec![0, 1],
                vec![1, 0],
                vec![0, 1],
                vec![1, 2],
                vec![2, 0],
                vec![3, 4]
            ]
        ),
        5
    );
    assert_eq!(
        maximum_requests(
            2,
            vec![
                vec![1, 1],
                vec![1, 0],
                vec![0, 1],
                vec![0, 0],
                vec![0, 0],
                vec![0, 1],
                vec![0, 1],
                vec![1, 0],
                vec![1, 0],
                vec![1, 1],
                vec![0, 0],
                vec![1, 0]
            ]
        ),
        11
    );
}

/*
 *
 * O(2^m), O(n+m)
 */
pub fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
    fn go(i: usize, dp: &mut Vec<i32>, size: i32, reqs: &[Vec<i32>]) -> i32 {
        let mut res = 0;
        if dp.iter().all(|&x| x == 0) {
            res = size;
        }

        for j in i..reqs.len() {
            dp[reqs[j][0] as usize] -= 1;
            dp[reqs[j][1] as usize] += 1;
            res = res.max(go(j + 1, dp, size + 1, reqs));
            dp[reqs[j][0] as usize] += 1;
            dp[reqs[j][1] as usize] -= 1;
        }
        res
    }

    go(0, &mut vec![0; n as usize], 0, &requests)
}
