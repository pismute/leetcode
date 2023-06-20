fn sort(mut xs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    xs.sort();
    xs
}

fn main() {
    assert_eq!(
        sort(combination_sum(vec![2, 3, 6, 7], 7)),
        sort(vec![vec![2, 2, 3], vec![7]])
    );
    assert_eq!(
        sort(combination_sum(vec![3, 5, 8], 11)),
        sort(vec![vec![3, 3, 5], vec![3, 8]])
    );
    assert_eq!(
        sort(combination_sum(vec![2, 3, 5], 8)),
        sort(vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]])
    );
    assert_eq!(
        sort(combination_sum(vec![2, 3, 6, 7], 5)),
        sort(vec![vec![2, 3]])
    );
    assert_eq!(combination_sum(vec![2], 1).is_empty(), true);
}

/*
 * [2, 3, 6, 7], 7
 *
 *              2;5                3;4     6;1    7;0
 *        [2;3       3;2]     [2;2   3;1]
 *   [2;1    3;0]  [2;0]      [2;0]
 *
 * brute force:
 * O(c^n), O(n)
 *
 * dp
 * dp[0] = [[]]
 * dp[1] = []
 * dp[2] = [[2]]
 * dp[3] = [[3]]
 * dp[4] = dp[4-2]*2 + dp[4-3]*3 = [[2,2]]
 * dp[5] = dp[5-2]*2 + dp[5-3]*3 = [[3,2], [2,3]]
 * dp[6] = dp[6-2]*2 + dp[6-3]*3 + dp[6-6]*6 = [[2,2,2], [3,3], [6]]
 * dp[7] = dp[7-2]*2 + dp[7-3]*3 + dp[7-6]*6 + dp[7-7]*7 = [[3,2,2], [2,3,2], [2,2,3], [7]]
 *
 * dp = vector of set of sorted vector
 * O(c*(n + nlogn)), O(n*c*n)
 *
 * backtracking: dfs
 *
 *              2;5                3;4     6;1    7;0
 *        [2;3       3;2]     [2;2;x   3;1]
 *   [2;1    3;0]  [2;0;x]      [2;0;x]
 *  x - don't go back to previous elements
 *                                  [];0
 *                          2;2            [];0
 *                   2;4         3;5    6;6    [];0
 *            2;6       3;7                  7;7   [];0
 *
 * O(2^n), O(n)
 */

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    fn go(
        i: usize,
        cur: i32,
        cs: &mut Vec<i32>,
        candies: &[i32],
        tar: i32,
        acc: &mut Vec<Vec<i32>>,
    ) {
        if cur == tar {
            acc.push(cs.clone());
        } else if i >= candies.len() || cur > tar {
        } else {
            let c = candies[i];
            cs.push(c);
            go(i, cur + c, cs, candies, tar, acc);
            cs.pop();
            go(i + 1, cur, cs, candies, tar, acc);
        }
    }

    let mut empty = vec![];
    let mut res = vec![];
    go(0, 0, &mut empty, &candidates, target, &mut res);
    res
}

// dp
// pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
//     type Set = std::collections::HashSet<Vec<i32>>;
//     type DP = Vec<Set>;
//     let mut dp: DP = vec![Set::with_capacity(0); target as usize + 1];

//     dp[0].insert(vec![]);
//     for i in 1..=target as usize {
//         let mut res = Set::new();
//         for c in &candidates {
//             if i >= *c as usize {
//                 for arr in &dp[i - *c as usize] {
//                     let mut new_arr = arr.clone();
//                     new_arr.push(*c);
//                     new_arr.sort();
//                     res.insert(new_arr);
//                 }
//             }
//         }
//         dp[i] = res;
//     }

//     let mut res = vec![];
//     for set in &dp[target as usize] {
//         res.push(set.into_iter().copied().collect());
//     }
//     res
// }
