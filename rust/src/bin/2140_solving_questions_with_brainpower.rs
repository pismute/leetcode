fn main() {
    assert_eq!(
        most_points(vec![
            vec![21, 5],
            vec![92, 3],
            vec![74, 2],
            vec![39, 4],
            vec![58, 2],
            vec![5, 5],
            vec![49, 4],
            vec![65, 3]
        ]),
        157
    );
    assert_eq!(
        most_points(vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]]),
        5
    );
    assert_eq!(
        most_points(vec![
            vec![1, 1],
            vec![2, 2],
            vec![3, 3],
            vec![4, 4],
            vec![5, 5]
        ]),
        7
    );
}

/*
[[3,2],[4,3],[4,4],[2,5]]

|    [3,2]:3   [4,3]:4    [4,4]:4   [2,5]:2
| [2,5]:5

brute force
o(2^n), O(n)

dp: bottom up
Array[index] = maximum of points

[5,4,4,2]

dp[i] = points + dp[i + brainpower + 1]

O(n), O(n)

*/
pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
    let mut dp = vec![0 as i64; questions.len() + 1];

    for i in (0..questions.len()).rev() {
        let points = questions[i][0];
        let brainpower = questions[i][1];

        let j = i + brainpower as usize + 1;
        dp[i] = dp[i + 1].max(points as i64 + if j < questions.len() { dp[j] } else { 0 });
    }

    dp[0]
}
